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

#[derive(Hash, Clone, Copy, PartialEq, Eq)]
pub enum GameCell {
    Empty,
    Box,
    Player,
    Sink,
    SinkWithBox,
    Unmovable,
}

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
        return result;
    }

    pub fn draw_sprite(self: &Self, kind: GameCell, x: f32, y: f32, size_x: f32, size_y: f32) {
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
}

impl GameBoard {
    pub async fn new() -> Self {
        Self {
            sprites: SpriteManager::new().await,
        }
    }
}

impl DrawGameState for GameBoard {
    fn draw_board(
        self: &GameBoard,
        game_state: &GameState,
        offset_x: f32,
        offset_y: f32,
        board_size: f32,
    ) {
        clear_background(LIGHTGRAY);

        // let game_size = screen_width().min(screen_height()) - 50.0;
        // let offset_x = (screen_width() - game_size) / 2. - 40.;
        // let offset_y = (screen_height() - game_size) / 2. + 50.;
        let sq_size = board_size / SQUARES as f32;

        draw_rectangle(offset_x, offset_y, board_size, board_size, WHITE);

        for i in 1..SQUARES {
            draw_line(
                offset_x,
                offset_y + sq_size * i as f32,
                screen_width() - offset_x,
                offset_y + sq_size * i as f32,
                2.,
                LIGHTGRAY,
            );
        }

        for i in 1..SQUARES {
            draw_line(
                offset_x + sq_size * i as f32,
                offset_y,
                offset_x + sq_size * i as f32,
                screen_height() - offset_y,
                2.,
                LIGHTGRAY,
            );
        }

        let draw_point = |p: &Point, kind: GameCell| {
            self.sprites.draw_sprite(
                kind,
                offset_x + p.x as f32 * sq_size,
                offset_y + p.y as f32 * sq_size,
                sq_size,
                sq_size,
            )
        };

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
        let steps = game_state.steps();
        draw_text(
            format!("Steps: {steps}").as_str(),
            10.,
            25. + board_size,
            20.,
            DARKGRAY,
        );
        draw_text(game_state.get_title(), 10., 50. + board_size, 20., DARKGRAY);
    }

    fn draw_gameover(self: &GameBoard) {
        clear_background(WHITE);
        let text = "Game Over. You won!";
        let font_size = 30.;
        let text_size = measure_text(text, None, font_size as _, 1.0);
        let sq_size = screen_height() / SQUARES as f32;
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
