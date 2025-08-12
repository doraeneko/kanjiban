// kanjiban
// (C) 2025 by JoAn
// Input control logic for game board, keys or swipe.
use crate::game_state::Point;
use macroquad::prelude::*;
use std::collections::HashMap;

pub struct InputControl<'a> {
    camera: &'a Camera2D,
    touch_starts: HashMap<u64, Vec2>,
}

pub const DIR_NO_MOVE: Point = Point { x: 0, y: 0 };
pub const DIR_UP: Point = Point { x: 0, y: -1 };
pub const DIR_DOWN: Point = Point { x: 0, y: 1 };
pub const DIR_RIGHT: Point = Point { x: 1, y: 0 };
pub const DIR_LEFT: Point = Point { x: -1, y: 0 };

impl<'a> InputControl<'a> {
    pub fn new(camera: &'a Camera2D) -> Self {
        Self {
            camera,
            touch_starts: HashMap::new(),
        }
    }
    // Returns the direction currently pressed (if any touch or click inside buttons)
    // Returns an Option with Direction enum, or None if no button pressed
    pub async fn get_direction(&mut self) -> Point {
        if is_key_down(KeyCode::Right) {
            return DIR_RIGHT;
        } else if is_key_down(KeyCode::Left) {
            return DIR_LEFT;
        } else if is_key_down(KeyCode::Up) {
            return DIR_UP;
        } else if is_key_down(KeyCode::Down) {
            return DIR_DOWN;
        }
        for touch in touches() {
            match touch.phase {
                TouchPhase::Started => {
                    self.touch_starts.insert(touch.id, touch.position);
                }
                TouchPhase::Ended => {
                    if let Some(start) = self.touch_starts.remove(&touch.id) {
                        let delta = touch.position - start;
                        if delta.length() > 20.0 {
                            if delta.x.abs() > delta.y.abs() {
                                if delta.x > 0.0 {
                                    return DIR_RIGHT;
                                } else {
                                    return DIR_LEFT;
                                }
                            } else if delta.y > 0.0 {
                                return DIR_DOWN;
                            } else {
                                return DIR_UP;
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        // found no direction request
        DIR_NO_MOVE
    }
}
