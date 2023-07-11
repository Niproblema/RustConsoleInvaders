use crate::{NUM_COLUMNS, NUM_ROWS, frame::Drawable};
use crate::frame::Frame;


pub struct Player {
    pos_x: usize,
    pos_y: usize,
}

impl Default for Player{
    fn default() -> Self {
        Self {
            pos_x: NUM_COLUMNS / 2,
            pos_y: NUM_ROWS - 1,
        }
    }
}

impl Player {
    pub fn new() -> Self {
        Self {
            pos_x: NUM_COLUMNS / 2,
            pos_y: NUM_ROWS - 1,
        }
    }

    pub fn move_left(&mut self) {
        if self.pos_x > 0 {
            self.pos_x -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.pos_x < NUM_COLUMNS - 1 {
            self.pos_x += 1;
        }
    }
}

impl Drawable for Player{
    fn draw(&self, frame: &mut Frame) {
        frame[self.pos_x][self.pos_y] = "A";
    }
}
