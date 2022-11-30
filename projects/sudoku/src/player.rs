use crate::{
    frame::{Drawable, Frame},
    {NUM_COLS, NUM_ROWS},
};
//use std::time::Duration;

pub struct Player {
    x: usize,
    y: usize,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: 5,
            y: 5,
        }
    }
    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
            if self.x == 3 || self.x == 7 {
                self.x -= 1;
            }
        }
    }
    pub fn move_right(&mut self) {
        if self.x < NUM_COLS - 1 {
            self.x += 1;
            if self.x == 3 || self.x == 7 {
                self.x += 1;
            }
        }
    }
    pub fn move_down(&mut self) {
        if self.y < NUM_ROWS - 1 {
            self.y += 1;
            if self.y == 3 || self.y == 7 {
                self.y += 1;
            }
        }
    }
    pub fn move_up(&mut self) {
        if self.y > 0 {
            self.y -= 1;
            if self.y == 3 || self.y == 7 {
                self.y -= 1;
            }
        }
    }
    pub fn insert(&mut self, frame: &mut Frame) {
        frame[self.x][self.y] = "X";
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = "_";
    }
}
