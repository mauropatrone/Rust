use sudoku_solver::{
    SIZE,
    board,
    rules,
};

fn main() {
    let mut board = create_board(0);
    let mut draft = board::create_draft(&board);
    println!("board:");
    for i in 0..SIZE {
        println!("{:?}", board::row(i,&board));
    }

    let mut count: u32 = 0;
    while !draft.is_empty() {
        let mut idx = 0;
        'a: while idx < draft.len() {
                rules::col_rule(&mut board, &mut draft, idx);
                if idx == draft.len() {
                    break 'a
                }
                rules::row_rule(&mut board, &mut draft, idx);
                if idx == draft.len() {
                    break 'a
                }
                if !rules::box_rule(&mut board, &mut draft, idx) {
                    idx += 1;
                }
                count += 1;
        }
    }

    println!("count: {}",count);
    println!("board:");
    for i in 0..SIZE {
        println!("{:?}",board::row(i,&board));
    }
}

fn create_board(index: usize) -> board::Board {
    /* Insert manually each number of the initial grid */
    /* Empty numbers must be inserted as zeros */
    let mut grid: Vec<Vec<u32>> = Vec::new();
    match index {
        0 => {
            //             0 1 2 3 4 5 6 7 8
            grid.push(vec![0,0,7,0,0,0,1,4,0]);//0
            grid.push(vec![4,0,3,0,9,7,0,0,0]);//1
            grid.push(vec![0,2,1,0,0,0,0,9,0]);//2
            grid.push(vec![0,0,0,0,1,8,0,7,4]);//3
            grid.push(vec![0,0,0,5,0,2,0,0,0]);//4
            grid.push(vec![5,7,0,4,3,0,0,0,0]);//5
            grid.push(vec![0,9,0,0,0,0,4,2,0]);//6
            grid.push(vec![0,0,0,9,5,0,8,0,6]);//7
            grid.push(vec![0,8,6,0,0,0,3,0,0]);//8
        },
        _ => {
            //             0 1 2 3 4 5 6 7 8
            grid.push(vec![0,0,0,0,0,0,0,1,0]);//0
            grid.push(vec![2,9,0,1,0,6,0,8,0]);//1
            grid.push(vec![0,6,4,8,0,0,0,0,0]);//2
            grid.push(vec![6,0,0,0,2,5,8,7,1]);//3
            grid.push(vec![0,0,0,0,0,0,0,0,0]);//4
            grid.push(vec![7,3,8,6,4,0,0,0,9]);//5
            grid.push(vec![0,0,0,0,0,4,2,5,0]);//6
            grid.push(vec![0,1,0,2,0,7,0,9,6]);//7
            grid.push(vec![0,7,0,0,0,0,0,0,0]);//8
        },
    }
    board::create_board(&grid) 
}

