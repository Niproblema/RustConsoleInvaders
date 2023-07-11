use std::io::{Stdout, Write};

use crossterm::{
    style::{Color, SetBackgroundColor},
    terminal::{Clear, ClearType},
    QueueableCommand, cursor::MoveTo,
};

use crate::{frame::Frame, NUM_COLUMNS, NUM_ROWS};

pub fn render(stdout: &mut Stdout, last_frame: &Frame, new_frame: &Frame, force: bool) {
    if force {
        stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    }

    for x in 0..NUM_COLUMNS {
        for y in 0..NUM_ROWS {
            if new_frame[x][y] != last_frame[x][y] || force {
                stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
                print!("{}", new_frame[x][y]);
            }
        }
    }

    stdout.flush().unwrap();
}
