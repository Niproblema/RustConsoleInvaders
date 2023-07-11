use std::time::Duration;

use rusty_time::Timer;

use crate::frame::Drawable;

pub struct Projectile {
    pub pos_x: usize,
    pub pos_y: usize,
    pub is_exploding: bool,
    timer: Timer,
}

impl Projectile {
    pub fn new(spawn_pos_x: usize, spawn_pos_y: usize) -> Self {
        Self {
            pos_x: spawn_pos_x,
            pos_y: spawn_pos_y,
            is_exploding: false,
            timer: Timer::from_millis(50),
        }
    }

    pub fn update(&mut self, time_delta: Duration) {
        self.timer.update(time_delta);
        if self.timer.ready && !self.is_exploding {
            if self.pos_y > 0 {
                self.pos_y -= 1;
            }
            self.timer.reset();
        }
    }

    pub fn explode(&mut self) {
        self.is_exploding = true;
        self.timer = Timer::from_millis(250);
    }

    pub fn is_dead(&self) -> bool {
        (self.is_exploding && self.timer.ready) || self.pos_y == 0
    }
}

impl Drawable for Projectile {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        frame[self.pos_x][self.pos_y] = if self.is_exploding { "*" } else { "|" }
    }
}
