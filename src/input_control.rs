// kanjiban
// (C) 2025 by JoAn
// Input control logic for game board, keys or swipe.

use crate::game_state::Point;
use macroquad::prelude::*;

pub struct InputControl {
    touch_start: Option<Vec2>,
}

pub const DIR_NO_MOVE: Point = Point { x: 0, y: 0 };
pub const DIR_UP: Point = Point { x: 0, y: -1 };
pub const DIR_DOWN: Point = Point { x: 0, y: 1 };
pub const DIR_RIGHT: Point = Point { x: 1, y: 0 };
pub const DIR_LEFT: Point = Point { x: -1, y: 0 };

impl InputControl {
    pub fn new() -> Self {
        Self { touch_start: None }
    }
    // Returns the direction currently pressed (if any touch or click inside buttons)
    // Returns an Option with Direction enum, or None if no button pressed
    pub fn get_direction(&mut self) -> Point {
        if is_key_pressed(KeyCode::Right) {
            return DIR_RIGHT;
        } else if is_key_pressed(KeyCode::Left) {
            return DIR_LEFT;
        } else if is_key_pressed(KeyCode::Up) {
            return DIR_UP;
        } else if is_key_pressed(KeyCode::Down) {
            return DIR_DOWN;
        }

        for touch in touches() {
            match touch.phase {
                TouchPhase::Started => {
                    self.touch_start = Some(touch.position);
                }
                TouchPhase::Ended => {
                    if let Some(start) = self.touch_start {
                        let delta = touch.position - start;
                        if delta.length() > 10.0 {
                            if delta.x.abs() > delta.y.abs() {
                                if delta.x > 0.0 {
                                    return DIR_RIGHT;
                                } else {
                                    return DIR_LEFT;
                                }
                            } else {
                                if delta.y > 0.0 {
                                    return DIR_DOWN;
                                }
                                return DIR_UP;
                            }
                        }
                    }
                    self.touch_start = None;
                }
                _ => {}
            }
        }
        DIR_NO_MOVE
    }
}
