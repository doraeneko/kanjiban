// kanjiban
// (C) 2025 by JoAn
// Level choCombo box functionality. Macroquad's combobox scales poorly.
// Chatgpt helped with this component a lot. But it was necessary to adapt to the camera position.

use crate::macroquad_helpers::{FontProvider, get_adjusted_mouse_position};
use macroquad::prelude::*;

pub struct LevelChooser<'a> {
    rect: Rect,
    items: &'static [&'static str],
    selected: usize,
    is_open: bool,
    camera: &'a Camera2D,
    fonts: &'a FontProvider,
}

impl<'a> LevelChooser<'a> {
    pub fn new(
        camera: &'a Camera2D,
        x: f32,
        y: f32,
        width: f32,
        items: &'static [&'static str],
        fonts: &'a FontProvider,
    ) -> Self {
        LevelChooser {
            rect: Rect::new(x, y, width, 80.0),
            items,
            selected: 0,
            is_open: false,
            camera,
            fonts,
        }
    }

    pub fn draw(&self) {
        // Draw the main box
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, DARKGRAY);
        let text_params = TextParams {
            font: Some(self.fonts.font()),
            font_size: 68,
            color: WHITE,
            ..Default::default()
        };
        let symbol_text_params = TextParams {
            font: Some(self.fonts.symbol_font()),
            font_size: 68,
            color: WHITE,
            ..Default::default()
        };
        draw_text_ex(
            "▼ Level",
            self.rect.x + 5.0,
            self.rect.y + 63.0,
            symbol_text_params.clone(),
        );

        let mouse_world = get_adjusted_mouse_position(self.camera);

        // If open, draw the dropdown items below
        if self.is_open {
            for (i, item) in self.items.iter().enumerate() {
                let y = self.rect.y + self.rect.h * (i as f32 + 1.0);
                let item_rect = Rect::new(self.rect.x, y, self.rect.w, self.rect.h);

                // Highlight on hover
                if item_rect.contains(mouse_world) {
                    draw_rectangle(item_rect.x, item_rect.y, item_rect.w, item_rect.h, GRAY);
                } else {
                    draw_rectangle(item_rect.x, item_rect.y, item_rect.w, item_rect.h, DARKGRAY);
                }

                draw_text_ex(
                    item,
                    item_rect.x + 5.0,
                    item_rect.y + 60.0,
                    text_params.clone(),
                );
            }
        }
    }

    pub fn update(&mut self) -> Option<usize> {
        let mouse_pos = get_adjusted_mouse_position(self.camera);

        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_pt = Vec2::new(mouse_pos.x, mouse_pos.y);

            if self.rect.contains(mouse_pt) {
                self.is_open = !self.is_open;
                return None;
            }

            if self.is_open {
                for i in 0..self.items.len() {
                    let item_rect = Rect::new(
                        self.rect.x,
                        self.rect.y + self.rect.h * (i as f32 + 1.0),
                        self.rect.w,
                        self.rect.h,
                    );

                    if item_rect.contains(mouse_pt) {
                        self.selected = i;
                        self.is_open = false;
                        return Some(i);
                    }
                }
                self.is_open = false;
            }
        }

        None
    }
}
