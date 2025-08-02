// kanjiban
// (C) 2025 by JoAn
// Drawing game states to the screen.

use crate::game_state::*;
use macroquad::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GameCell {
    Empty,
    Player,
    Unmovable,
    Movable,
    Sink,
}

pub struct GameBoard {
    pub width: usize,
    pub height: usize,
    cells: Vec<GameCell>,
}

impl GameBoard {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![GameCell::Empty; width * height],
        }
    }

    pub fn set(&mut self, pos: &Point, value: GameCell) {
        if pos.x >= 0
            && pos.y >= 0
            && (pos.x as usize) < self.width
            && (pos.y as usize) < self.height
        {
            self.cells[(pos.x as usize) * self.width + (pos.y as usize)] = value;
        }
    }

    pub fn get(&self, pos: &Point) -> GameCell {
        if (pos.x >= 0)
            && (pos.y >= 0)
            && (pos.x as usize) < self.width
            && (pos.y as usize) < self.height
        {
            self.cells[(pos.x as usize) * self.width + (pos.y as usize)]
        } else {
            GameCell::Empty
        }
    }
}

impl DrawGameState for GameBoard {
    // render all blocks, player and step counter
    fn draw_board(self: &GameBoard, game_state: &GameState) {
        clear_background(LIGHTGRAY);

        let game_size = screen_width().min(screen_height());
        let offset_x = (screen_width() - game_size) / 2. + 10.;
        let offset_y = (screen_height() - game_size) / 2. + 10.;
        let sq_size = (screen_height() - offset_y * 2.) / SQUARES as f32;

        draw_rectangle(offset_x, offset_y, game_size - 20., game_size - 20., WHITE);

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

        for s in &game_state.sinks {
            draw_rectangle(
                offset_x + s.x as f32 * sq_size,
                offset_y + s.y as f32 * sq_size,
                sq_size,
                sq_size,
                GREEN,
            );
        }

        for b in &game_state.unmovable_blocks {
            draw_rectangle(
                offset_x + b.x as f32 * sq_size,
                offset_y + b.y as f32 * sq_size,
                sq_size,
                sq_size,
                RED,
            );
        }

        for b in &game_state.movable_blocks {
            draw_rectangle(
                offset_x + b.x as f32 * sq_size,
                offset_y + b.y as f32 * sq_size,
                sq_size,
                sq_size,
                GOLD,
            );
        }

        draw_rectangle(
            offset_x + game_state.get_player_position().x as f32 * sq_size,
            offset_y + game_state.get_player_position().y as f32 * sq_size,
            sq_size,
            sq_size,
            DARKGREEN,
        );
        let steps = game_state.steps();
        draw_text(format!("Steps: {steps}").as_str(), 10., 20., 20., DARKGRAY);
    }

    fn draw_gameover(self: &GameBoard) {
        clear_background(WHITE);
        let text = "Game Over. ";
        let font_size = 30.;
        let text_size = measure_text(text, None, font_size as _, 1.0);

        draw_text(
            text,
            screen_width() / 2. - text_size.width / 2.,
            screen_height() / 2. + text_size.height / 2.,
            font_size,
            DARKGRAY,
        );
    }
}
