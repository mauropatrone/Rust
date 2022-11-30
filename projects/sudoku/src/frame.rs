use crate::{NUM_COLS, NUM_ROWS};

pub type Frame = Vec<Vec<& 'static str>>;

pub fn new_frame() -> Frame{
    let mut cols = Vec::with_capacity(NUM_COLS);
    for c in 0..NUM_COLS{
        let mut col = Vec::with_capacity(NUM_ROWS);
        for r in 0..NUM_ROWS {
            if c == 3 || c == 7 {
                if r == 3 || r == 7{
                    col.push("+");
                } else {
                    col.push("|");
                }               
            } else if r == 3 || r == 7 {
                    col.push("-");
            } else {
                col.push(" ");
            }
        }
        cols.push(col);
    }
    cols
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}