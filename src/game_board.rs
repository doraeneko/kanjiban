// kanjiban
// (C) 2025 by JoAn
// Drawing game states to the screen using a game board.

use crate::game_state::*;
use macroquad::prelude::*;
use std::collections::HashMap;

const SPRITES_DIR: &str = "assets/sprites/";
const PLAYER_SPRITE: &str = "player.png";
const BOX_SPRITE: &str = "box.png";
const UNMOVABLE_SPRITE: &str = "silver_plate.png";
const SINK_SPRITE: &str = "target_plate.png";
const SINK_WITH_BOX_SPRITE: &str = "saved_box.png";
const EMPTY_SPRITE: &str = "empty.png";

pub struct SpriteManager {
    sprites: HashMap<GameCell, Texture2D>,
}

impl SpriteManager {
    pub async fn new() -> SpriteManager {
        let mut result = SpriteManager {
            sprites: HashMap::new(),
        };
        let sprite_info: [(&str, GameCell); 6] = [
            (EMPTY_SPRITE, GameCell::Empty),
            (BOX_SPRITE, GameCell::Box),
            (PLAYER_SPRITE, GameCell::Player),
            (UNMOVABLE_SPRITE, GameCell::Unmovable),
            (SINK_SPRITE, GameCell::Sink),
            (SINK_WITH_BOX_SPRITE, GameCell::SinkWithBox),
        ];
        for (sprite_path, cell_element) in sprite_info {
            let texture = load_texture(&[SPRITES_DIR, sprite_path].concat())
                .await
                .unwrap();
            result.sprites.insert(cell_element, texture);
        }
        result
    }

    pub fn draw_sprite(&self, kind: GameCell, x: f32, y: f32, size_x: f32, size_y: f32) {
        draw_texture_ex(
            &self.sprites[&kind],
            x,
            y,
            WHITE, // tint color
            DrawTextureParams {
                dest_size: Some(vec2(size_x, size_y)),
                ..Default::default()
            },
        );
    }
}

pub struct GameBoard {
    sprites: SpriteManager,
    top_x: f32,
    top_y: f32,
    max_x: f32,
    max_y: f32,
}

impl GameBoard {
    pub async fn new(top_x: f32, top_y: f32, max_x: f32, max_y: f32) -> Self {
        Self {
            sprites: SpriteManager::new().await,
            top_x,
            top_y,
            max_x,
            max_y,
        }
    }
}

impl GameBoard {
    pub fn draw_board(self: &GameBoard, game_state: &GameState) {
        clear_background(LIGHTGRAY);

        let sq_size =
            (self.max_x.min(self.max_y)) / (game_state.height().min(game_state.width()) as f32);

        draw_rectangle(self.top_x, self.top_y, self.max_x, self.max_y, BEIGE);

        let draw_point = |p: &Point, kind: GameCell| {
            self.sprites.draw_sprite(
                kind,
                self.top_x + p.x as f32 * sq_size,
                self.top_y + p.y as f32 * sq_size,
                sq_size,
                sq_size,
            )
        };
        for x in 0..game_state.width() {
            for y in 0..game_state.height() {
                draw_point(
                    &Point {
                        x: x as i32,
                        y: y as i32,
                    },
                    GameCell::Empty,
                );
            }
        }
        for s in &game_state.sinks {
            draw_point(s, GameCell::Sink);
        }
        for b in &game_state.unmovable_blocks {
            draw_point(b, GameCell::Unmovable);
        }
        for b in &game_state.movable_blocks {
            if game_state.is_target(b) {
                draw_point(b, GameCell::SinkWithBox);
            } else {
                draw_point(b, GameCell::Box);
            }
        }
        draw_point(&game_state.get_player_position(), GameCell::Player);
    }

    pub fn draw_win(self: &GameBoard, game_state: &GameState) {
        self.draw_board(game_state);
        clear_background(WHITE);
        let text = "You won!";
        let font_size = 30.;
        let text_size = measure_text(text, None, font_size as _, 1.0);
        let sq_size = screen_height() / game_state.width().max(game_state.height()) as f32;
        self.sprites.draw_sprite(
            GameCell::Player,
            screen_width() / 2. - text_size.width / 2. - 70.0,
            screen_height() / 2. + text_size.height / 2.,
            sq_size,
            sq_size,
        );
        draw_text(
            text,
            screen_width() / 2. - text_size.width / 2.,
            screen_height() / 2. + text_size.height / 2.,
            font_size,
            DARKGRAY,
        );
    }
}
