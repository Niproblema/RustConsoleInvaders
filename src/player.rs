use std::time::Duration;

use rusty_time::Timer;

use crate::frame::Frame;
use crate::projectile::{self, Projectile};
use crate::{frame::Drawable, NUM_COLUMNS, NUM_ROWS};

pub struct Player {
    pos_x: usize,
    pos_y: usize,
    projectiles: Vec<Projectile>,
    reload_timer: Timer,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            pos_x: NUM_COLUMNS / 2,
            pos_y: NUM_ROWS - 1,
            projectiles: Vec::new(),
            reload_timer: Timer::from_millis(200),
        }
    }
}

impl Player {
    pub fn new() -> Self {
        Self {
            pos_x: NUM_COLUMNS / 2,
            pos_y: NUM_ROWS - 1,
            projectiles: Vec::new(),
            reload_timer: Timer::from_millis(200),
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

    pub fn fire(&mut self) -> bool {
        if self.reload_timer.ready {
            self.projectiles
                .push(Projectile::new(self.pos_x, self.pos_y));
            self.reload_timer.reset();
            true
        } else {
            false
        }
    }

    pub fn update(&mut self, time_delta: Duration) {
        self.reload_timer.update(time_delta);
        self.update_projectiles(time_delta);
    }

    fn update_projectiles(&mut self, time_delta: Duration) {
        for projectile in self.projectiles.iter_mut() {
            projectile.update(time_delta)
        }

        self.projectiles.retain(|projectile| !projectile.is_dead());
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        for projectile in self.projectiles.iter() {
            projectile.draw(frame);
        }

        frame[self.pos_x][self.pos_y] = "A";
    }
}
