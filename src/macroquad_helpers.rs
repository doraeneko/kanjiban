// kanjiban
// (C) 2025 by JoAn
// Macroquad helper functions.

use macroquad::prelude::*;

pub fn get_adjusted_mouse_position(camera: &Camera2D) -> Vec2 {
    let mouse_screen = mouse_position();
    let mouse_screen_vec2 = Vec2::new(mouse_screen.0, mouse_screen.1);
    camera.screen_to_world(mouse_screen_vec2)
}
