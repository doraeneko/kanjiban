// kanjiban
// (C) 2025 by JoAn
// Input control logic for game board, keys or swipe.
use crate::game_state::Point;
use macroquad::prelude::*;
use std::collections::HashMap;

pub struct InputControl<'a> {
    camera: &'a Camera2D,
    touch_starts: HashMap<u64, Vec2>,
    prev_touches: HashMap<u64, Vec2>,
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
            prev_touches: HashMap::new(),
        }
    }
    // Returns the direction currently pressed (if any touch or click inside buttons)
    // Returns an Option with Direction enum, or None if no button pressed
    pub fn get_direction(&mut self) -> Point {
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
            let (fill_color, size) = match touch.phase {
                TouchPhase::Started => (GREEN, 80.0),
                TouchPhase::Stationary => (WHITE, 60.0),
                TouchPhase::Moved => (YELLOW, 60.0),
                TouchPhase::Ended => (BLUE, 80.0),
                TouchPhase::Cancelled => (BLACK, 80.0),
            };
            draw_circle(touch.position.x, touch.position.y, size, fill_color);
        }
        // Get current touches
        let current_touches: Vec<Touch> = touches();

        // Register new touches
        for touch in &current_touches {
            if !self.touch_starts.contains_key(&touch.id) {
                self.touch_starts.insert(touch.id, touch.position);
            }
        }

        // Detect disappeared touches
        for (&id, &start_pos) in self.prev_touches.iter() {
            if !current_touches.iter().any(|t| t.id == id) {
                // This touch disappeared — treat as "ended"
                if let Some(start) = self.touch_starts.remove(&id) {
                    let delta = start_pos - start; // careful: might want end_pos - start_pos
                    let end_pos = start_pos;
                    let delta = end_pos - start;

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
        }

        // Update prev_touches for next frame
        self.prev_touches.clear();
        for touch in &current_touches {
            self.prev_touches.insert(touch.id, touch.position);
        }

        // found no direction request
        DIR_NO_MOVE
    }
}
