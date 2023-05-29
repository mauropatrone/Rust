use crate::SIZE;

pub type Board = Vec<Vec<u32>>;

pub fn create_board(v: &Vec<Vec<u32>>) -> Board {
    let mut board = empty_board();
    for (y,r) in v.iter().enumerate() {
        for (x,val) in r.iter().enumerate() {
            if *val != 0 { 
                board[x][y]=*val;
            }
        }
    }
    board
}

fn empty_board() -> Board {
    let mut cols = Vec::with_capacity(SIZE);
    for _ in 0..SIZE{
        let col = vec![0; SIZE];
        cols.push(col);
    }
    cols
}

pub fn col(index: usize, board: &Board) -> Vec<u32> {
    board[index].clone()
}

pub fn row(index: usize, board: &Board) -> Vec<u32> {
    let mut v = Vec::with_capacity(SIZE);
    for i in 0..SIZE {
        v.push(board[i][index])
    }
    v
}

pub fn rec(index: usize, board: &Board) -> Vec<u32> {
    let (x,y) = match index {
        0 => (0,0),
        1 => (3,0),
        2 => (6,0),
        3 => (0,3),
        4 => (3,3),
        5 => (6,3),
        6 => (0,6),
        7 => (3,6),
        _ => (6,6),
    };
    [&board[x][y..y+3], &board[x+1][y..y+3], &board[x+2][y..y+3]].concat()
}

#[derive(Debug)]
pub struct Square {
    pub x: usize,
    pub y: usize,
    pub mark: Vec<u32>,
}

impl Square {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            mark: vec![1,2,3,4,5,6,7,8,9],
        }
    }
}

pub fn create_draft(board: &Board) -> Vec<Square> {
    let mut draft: Vec<Square> = Vec::new();
    for x in 0..SIZE{
        for y in 0..SIZE{
            if board[x][y] == 0 {
                let sq = Square::new(x,y);
                draft.push(sq)
            }
        }
    }
    draft
}