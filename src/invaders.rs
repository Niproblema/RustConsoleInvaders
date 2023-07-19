use std::{cmp::max, time::Duration};

use rusty_time::Timer;

use crate::{frame::Drawable, NUM_COLUMNS, NUM_ROWS};

const HORIZONTAL_PADDING: usize = 3;
const TOP_PADDING: usize = 1;
const UNIT_ROW_COUNT: usize = NUM_ROWS / 4;
const UNIT_COLUMN_SPACING: usize = 1;

pub struct Invader {
    pub pos_x: usize,
    pub pos_y: usize,
}

pub struct Invaders {
    pub units: Vec<Invader>,
    move_timer: Timer,
    move_step: isize,
}

impl Invaders {
    pub fn new() -> Self {
        let mut units = Vec::new();
        let mut last_x = 0;
        for x in HORIZONTAL_PADDING..NUM_COLUMNS - HORIZONTAL_PADDING {
            if last_x != 0 && x - last_x <= UNIT_COLUMN_SPACING {
                continue;
            }
            last_x = x;

            for y in TOP_PADDING..UNIT_ROW_COUNT {
                units.push(Invader { pos_x: x, pos_y: y })
            }
        }

        Self {
            units,
            move_timer: Timer::from_millis(500),
            move_step: 1,
        }
    }

    pub fn update(&mut self, time_delta: Duration) -> bool {
        self.move_timer.update(time_delta);
        if self.move_timer.ready {
            self.move_timer.reset();

            let mut move_down = false;

            // If moving left
            if self.move_step < 0 {
                if self.units.first().unwrap().pos_x < self.move_step.unsigned_abs() {
                    move_down = true;
                    self.move_step *= -1;
                }
            }
            // If moving right.
            else if self.units.last().unwrap().pos_x
                >= NUM_COLUMNS - self.move_step.unsigned_abs()
            {
                move_down = true;
                self.move_step *= -1;
            }

            if move_down {
                // Speed up movement with each row.
                let new_move_duration = max(self.move_timer.duration.as_millis() - 100, 100);
                self.move_timer = Timer::from_millis(new_move_duration as u64);
                self.move_vertical();
            } else {
                self.move_horizontal();
            }

            true
        } else {
            false
        }
    }

    pub fn are_all_defeated(&self) -> bool {
        self.units.is_empty()
    }

    pub fn reached_bottom(&self) -> bool {
        self.units.iter().map(|unit| unit.pos_y).max().unwrap_or(0) >= NUM_ROWS - 1
    }

    pub fn shoot_unit_at(&mut self, x: usize, y: usize) -> bool {
        if self.units.is_empty() {
            return false;
        }

        if let Some(unit_index) = self
            .units
            .iter()
            .position(|unit| unit.pos_x == x && unit.pos_y == y)
        {
            self.units.remove(unit_index);
            true
        } else {
            false
        }
    }

    fn move_vertical(&mut self) {
        for unit in self.units.iter_mut() {
            unit.pos_y += 1;
        }
    }

    fn move_horizontal(&mut self) {
        for unit in self.units.iter_mut() {
            unit.pos_x = unit.pos_x.checked_add_signed(self.move_step).unwrap();
        }
    }
}

impl Default for Invaders {
    fn default() -> Self {
        Self::new()
    }
}

impl Drawable for Invaders {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        for unit in self.units.iter() {
            // TODO replace with emoji
            frame[unit.pos_x][unit.pos_y] = "x"
        }
    }
}
