fn main() {
    let board = [
        "         ",
        "         ",
        "  *      ",
        "*        ",
        "       * ",
        "        *",
        "       * ",
        " *   * * ",
        " * *     "
    ];
    println!("puzzle:");
    for row in board {
        println!("{:?}",row);
    }
    println!("solution:");
    let mut sol = mines(&board);
    for row in sol {
        println!("{:?}",row);
    }
}



fn mines(board: &[&str]) -> Vec<String> {
    let mut sol = init(board);
    clean_board(&mut sol);
    match get_mines_indexes(&sol) {
        None => (),
        Some(mine_map) => {
            for mine in mine_map {
                update_neighbors(mine, &mut sol);
            }
        },
    }
    sol
}

fn get_mines_indexes(board: &Vec<String>) -> Option<Vec<(usize,usize)>>{
    let mut row_index: usize = 0;
    let mut v: Vec<(usize,usize)> = Vec::new();
    for row in board {
        match search_row(&row) {
            Some(x) => {
                for i in x {
                    v.push((row_index,i))
                }
            },
            None => (),
        }
        row_index += 1;
    }
    if v.is_empty() {
        return None
    }
    Some(v)
}

#[test]
fn test_get_mines_indexes() {
    let board = [
        "   ",
        "1 1",
        " 1 "
    ];
    let mut b = init(&board);
    assert_eq!(get_mines_indexes(&b),None);

    let board = [
        "   ",
        "1*1",
        "*1*"
    ];
    b = init(&board);
    assert_eq!(get_mines_indexes(&b),Some(vec![(1,1),(2,0),(2,2)]));
}

fn search_row(row: &String) -> Option<Vec<usize>> {
    let mut v: Vec<usize> = Vec::new();
    for index in 0..row.len() {
        match row.chars().nth(index).unwrap() {
            '*' => v.push(index),
            _ => (),
        }
    }
    if v.is_empty() {
        return None
    }
    Some(v)
}

#[test]
fn test_search_row() {
    let row = "1**";
    let s = String::from(row);
    assert_eq!(search_row(&s),Some(vec![1,2]));
    let row = "1 *";
    let s = String::from(row);
    assert_eq!(search_row(&s),Some(vec![2]));
    let row = "1  ";
    let s = String::from(row);
    assert_eq!(search_row(&s),None);
}

fn update_neighbors(center: (usize, usize), board: &mut Vec<String>) {
    let (ini, fin) = find_corners(center, board.len(), board[0].len());
    
    for row in ini.0..=fin.0 {
        for col in ini.1..=fin.1 {
            add(col,&mut board[row]);
        }
    }
}

#[test]
fn test_update_neighbors() {
    let board = [
        "   ",
        " * ",
        "   "
    ];
    let solved = [
        "111",
        "1*1",
        "111"
    ];
    let solved = [
        "11 ",
        "1* ",
        "   "
    ];
    let mut s: Vec<String> = Vec::new();
    s.push(String::from(solved[0]));
    s.push(String::from(solved[1]));
    s.push(String::from(solved[2]));

    let mut b = init(&board);
    let center: (usize, usize) = (0,0);
    update_neighbors(center, &mut b);
    assert_eq!(b,s);
}



fn find_corners(center: (usize, usize), num_row: usize, num_col: usize) -> ((usize,usize),(usize,usize)) {
    let mut ini: (usize, usize) = (0,0);
    let mut fin: (usize, usize) = (0,0);
    match center.0 {
        0 => fin.0 = 1.min(num_row - 1),
        _ => {
            ini.0 = center.0 - 1;
            fin.0 = (center.0 + 1).min(num_row - 1);
        },
    }
    match center.1 {
        0 => fin.1 = 1.min(num_col - 1),
        _ => {
            ini.1 = center.1 - 1;
            fin.1 = (center.1 + 1).min(num_col - 1)
        },
    }
    (ini,fin)
}

#[test]
fn test_find_corners() {
    let board = [
        "   ",
        "1*1",
        "111"
    ];
    let b = init(&board);
    let center: (usize, usize) = (1,1);
    let size = b.len();
    assert_eq!(find_corners(center, size, size),((0,0),(2,2)));
    
    //"*  "
    let center: (usize, usize) = (0,2);
    assert_eq!(find_corners(center, 1, 3),((0,1),(0,2)));

    //"*"
    //" "
    //" "
    let center: (usize, usize) = (1,0);
    assert_eq!(find_corners(center, 3, 1),((0,0),(2,0)));

}

fn add(index: usize, c: &mut String) {
    match c.chars().nth(index).unwrap() {
        ' ' => {
            c.remove(index);
            c.insert(index,'1');
        },
        '1' => {
            c.remove(index);
            c.insert(index,'2');
        },
        '2' => {
            c.remove(index);
            c.insert(index,'3');
        },
        '3' => {
            c.remove(index);
            c.insert(index,'4');
        },
        '4' => {
            c.remove(index);
            c.insert(index,'5');
        },
        '5' => {
            c.remove(index);
            c.insert(index,'6');
        },
        '6' => {
            c.remove(index);
            c.insert(index,'7');
        },
        '7' => {
            c.remove(index);
            c.insert(index,'8');
        },
        _ => (),
    }
}

#[test]
fn test_add() {
    let c = "1 *";
    let mut s = String::from(c);
    add(0,&mut s);
    assert_eq!(s,"2 *");
    add(1,&mut s);
    assert_eq!(s,"21*");
    add(2,&mut s);
    assert_eq!(s,"21*");
}

fn init(board: &[&str]) -> Vec<String>{
    let mut s: Vec<String> = Vec::new();
    for x in board {
        s.push(String::from(*x))
    }
    s
}

#[test]
fn test_init() {
    let board = [
        "   ",
        "1*1",
        "111"
    ];
    let mut s: Vec<String> = Vec::new();
    s.push(String::from(board[0]));
    s.push(String::from(board[1]));
    s.push(String::from(board[2]));
    assert_eq!(init(&board),s);
}

fn clean_board(board: &mut Vec<String>) {
    for row in board {
        for col in 0..row.len() {
            clean(col,row);
        }
    }
}

#[test]
fn test_clean_board() {
    let board = [
        "   ",
        "1*1",
        "111"
    ];
    let cleaned = [
        "   ",
        " * ",
        "   "
    ];
    let mut s: Vec<String> = Vec::new();
    s.push(String::from(cleaned[0]));
    s.push(String::from(cleaned[1]));
    s.push(String::from(cleaned[2]));
    
    let mut b = init(&board);
    clean_board(&mut b);
    assert_eq!(b,s);
}

fn clean(index: usize, c: &mut String) {
    match c.chars().nth(index).unwrap() {
        '*' | ' ' => (),
        _ => {
            c.remove(index);
            c.insert(index,' ');
        },
    }
}

#[test]
fn test_clean() {
    let c = "12*";
    let mut s = String::from(c);
    clean(0,&mut s);
    assert_eq!(s," 2*");
    clean(1,&mut s);
    assert_eq!(s,"  *");
    clean(2,&mut s);
    assert_eq!(s,"  *");
}