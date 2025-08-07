// kanjiban
// (C) 2025 by JoAn
// Pad with direction keys for phone usage.
// Chatgpt helped with this component a lot. We combined its logic with the keyboard inputs.

use crate::game_state::Point;
use crate::macroquad_helpers::get_adjusted_mouse_position;
use macroquad::prelude::*;

pub struct DirPad<'a> {
    camera: &'a Camera2D,
    center: Vec2,
    size: f32, // Size of the D-pad (e.g. 150px)
}

pub const DIR_NO_MOVE: Point = Point { x: 0, y: 0 };
pub const DIR_UP: Point = Point { x: 0, y: -1 };
pub const DIR_DOWN: Point = Point { x: 0, y: 1 };
pub const DIR_RIGHT: Point = Point { x: 1, y: 0 };
pub const DIR_LEFT: Point = Point { x: -1, y: 0 };

impl<'a> DirPad<'a> {
    pub fn new(camera: &'a Camera2D, center_x: f32, center_y: f32, size: f32) -> Self {
        Self {
            camera,
            center: Vec2 {
                x: center_x,
                y: center_y,
            },
            size,
        }
    }

    pub fn draw(&self) {
        let half = self.size / 2.0;
        let btn_size = self.size / 3.0;

        // Arrow button rectangles (Up, Down, Left, Right)
        let up = Rect::new(
            self.center.x - btn_size / 2.0,
            self.center.y - half,
            btn_size,
            btn_size,
        );
        let down = Rect::new(
            self.center.x - btn_size / 2.0,
            self.center.y + half - btn_size,
            btn_size,
            btn_size,
        );
        let left = Rect::new(
            self.center.x - half,
            self.center.y - btn_size / 2.0,
            btn_size,
            btn_size,
        );
        let right = Rect::new(
            self.center.x + half - btn_size,
            self.center.y - btn_size / 2.0,
            btn_size,
            btn_size,
        );

        // Colors depending on hover or touch
        for rect in [&up, &down, &left, &right] {
            let mouse_pos = get_adjusted_mouse_position(self.camera);
            let color = if rect.contains(mouse_pos) {
                SKYBLUE
            } else {
                DARKGRAY
            };
            draw_rectangle(rect.x, rect.y, rect.w, rect.h, color);
        }

        // Draw arrows (simple triangles)
        // Up
        draw_triangle(
            vec2(up.x + up.w / 2.0, up.y + 5.0),
            vec2(up.x + 5.0, up.y + up.h - 5.0),
            vec2(up.x + up.w - 5.0, up.y + up.h - 5.0),
            WHITE,
        );
        // Down
        draw_triangle(
            vec2(down.x + 5.0, down.y + 5.0),
            vec2(down.x + down.w - 5.0, down.y + 5.0),
            vec2(down.x + down.w / 2.0, down.y + down.h - 5.0),
            WHITE,
        );
        // Left
        draw_triangle(
            vec2(left.x + 5.0, left.y + left.h / 2.0),
            vec2(left.x + left.w - 5.0, left.y + 5.0),
            vec2(left.x + left.w - 5.0, left.y + left.h - 5.0),
            WHITE,
        );
        // Right
        draw_triangle(
            vec2(right.x + right.w - 5.0, right.y + right.h / 2.0),
            vec2(right.x + 5.0, right.y + 5.0),
            vec2(right.x + 5.0, right.y + right.h - 5.0),
            WHITE,
        );
    }

    // Returns the direction currently pressed (if any touch or click inside buttons)
    // Returns an Option with Direction enum, or None if no button pressed
    pub async fn get_direction(&self) -> Point {
        if is_key_down(KeyCode::Right) {
            return DIR_RIGHT;
        } else if is_key_down(KeyCode::Left) {
            return DIR_LEFT;
        } else if is_key_down(KeyCode::Up) {
            return DIR_UP;
        } else if is_key_down(KeyCode::Down) {
            return DIR_DOWN;
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            // now: for mobile use touches, else use mouse
            let pos = get_adjusted_mouse_position(self.camera);

            let half = self.size / 2.0;
            let btn_size = self.size / 3.0;

            let up = Rect::new(
                self.center.x - btn_size / 2.0,
                self.center.y - half,
                btn_size,
                btn_size,
            );
            let down = Rect::new(
                self.center.x - btn_size / 2.0,
                self.center.y + half - btn_size,
                btn_size,
                btn_size,
            );
            let left = Rect::new(
                self.center.x - half,
                self.center.y - btn_size / 2.0,
                btn_size,
                btn_size,
            );
            let right = Rect::new(
                self.center.x + half - btn_size,
                self.center.y - btn_size / 2.0,
                btn_size,
                btn_size,
            );

            if up.contains(pos) {
                return DIR_UP;
            } else if down.contains(pos) {
                return DIR_DOWN;
            } else if left.contains(pos) {
                return DIR_LEFT;
            } else if right.contains(pos) {
                return DIR_RIGHT;
            }
        }
        // found no direction request
        return DIR_NO_MOVE;
    }
}
