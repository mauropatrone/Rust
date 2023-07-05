fn main() {
    let board = [
        "   ",
        "1*1",
        "111"
    ];
    println!("{:?}",init(&board));
    let a = '*';
    let n = "   ".chars().count();
    let mut c = "*1".chars();
    println!("{n}");
    //println!("{}",&c[0..1]);
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

// fn mines(board: &[&str]) {
//     let mut sol = init(&board);
//     for row in 0..board.len() {
//         for col in 0..row.len() {
//             match row.chars().nth(index).unwrap() {
//                 '*' => return Some(index),
//                 _ => (),
//             }
//         }
//     }
// }

fn search_row(row: &String) -> Option<usize> {
    for index in 0..row.len() {
        match row.chars().nth(index).unwrap() {
            '*' => return Some(index),
            _ => (),
        }
    }
    None
}

#[test]
fn test_search_row() {
    let row = "1 *";
    let s = String::from(row);
    assert_eq!(search_row(&s),Some(2));
}

fn update_neighbors(center: (usize, usize), board: &mut Vec<String>) {
    let (ini, fin) = find_corners(center, board.len());
    
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



fn find_corners(center: (usize, usize), size: usize) -> ((usize,usize),(usize,usize)) {
    let mut ini: (usize, usize) = (0,0);
    let mut fin: (usize, usize) = (0,0);
    match center.0 {
        0 => fin.0 = 1,
        _ => {
            ini.0 = center.0 - 1;
            fin.0 = (center.0 + 1).min(size - 1);
        },
    }
    match center.1 {
        0 => fin.1 = 1,
        _ => {
            ini.1 = center.1 - 1;
            fin.1 = (center.1 + 1).min(size - 1)
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
    assert_eq!(find_corners(center, size),((0,0),(2,2)));
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