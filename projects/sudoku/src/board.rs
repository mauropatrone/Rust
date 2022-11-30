pub type Board: Vec<Square>;

pub fn new_board() -> Board {
    let mut board: Vec<Square> = Vec::new();
    let mut square = Square::new();
}

pub fn insert(x: usize, y: usize, val: u32) {
    square.insert(x,y,val);
    board.push(square);
}

struct Square {
    x: usize,
    y: usize,
    value: u32,
}

impl Square {
    fn new() -> Self {
        Self {
            x,
            y,
            value,
        }
    }
    fn insert(&mut self, x: usize, y: usize, val: u32) {
        self.x = x;
        self.y = y;
        self.value = val;
    }
}