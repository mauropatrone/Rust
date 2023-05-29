use crate::board::*;

pub fn col_rule(board: &mut Board, draft: &mut Vec<Square>, index: usize) {
    remove_repeated(&col(draft[index].x, &board), &mut draft[index]);
    if draft[index].mark.len() == 1 {
        board[draft[index].x][draft[index].y] = draft[index].mark[0];
        draft.remove(index);
    }
}

pub fn row_rule(board: &mut Board, draft: &mut Vec<Square>, index: usize) {
    remove_repeated(&row(draft[index].y, &board), &mut draft[index]);
    if draft[index].mark.len() == 1 {
        board[draft[index].x][draft[index].y] = draft[index].mark[0];
        draft.remove(index);
    }
}

pub fn box_rule(board: &mut Board, draft: &mut Vec<Square>, index: usize) -> bool {
    let box_index = match draft[index].x {
        0 | 1 | 2 => match draft[index].y {
            0 | 1 | 2 => 0,
            3 | 4 | 5 => 3,
            _ => 6,
        },
        3 | 4 | 5 => match draft[index].y {
            0 | 1 | 2 => 1,
            3 | 4 | 5 => 4,
            _ => 7,
        },
        _ => match draft[index].y {
            0 | 1 | 2 => 2,
            3 | 4 | 5 => 5,
            _ => 8,
        },
    };
    remove_repeated(&rec(box_index, &board), &mut draft[index]);
    if draft[index].mark.len() == 1 {
        board[draft[index].x][draft[index].y] = draft[index].mark[0];
        draft.remove(index);
        true
    } else {
        false
    }
}

fn remove_repeated(v: &Vec<u32>, square: &mut Square) {
    let mut i = 0;
    while i < square.mark.len() {
        if v.contains(&square.mark[i]) {
            square.mark.remove(i);
        } else {
            i += 1;
        }
    }
}