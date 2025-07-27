pub mod draw {

use macroquad::prelude::*;
use std::collections::LinkedList;
pub use crate::game_state::board::Point;
pub use crate::game_state::board::SQUARES;
pub use crate::game_state::board::Player;

// render all blocks, player and step counter
pub fn draw_game(
    unmovable_blocks: &mut LinkedList<Point>,
    movable_blocks: &mut LinkedList<Point>,
    sinks: &mut LinkedList<Point>,
    andi: &Player,
    steps: i16,
) {
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

    for s in sinks {
        draw_rectangle(
            offset_x + s.0 as f32 * sq_size,
            offset_y + s.1 as f32 * sq_size,
            sq_size,
            sq_size,
            GREEN,
        );
    }

    for b in unmovable_blocks {
        draw_rectangle(
            offset_x + b.0 as f32 * sq_size,
            offset_y + b.1 as f32 * sq_size,
            sq_size,
            sq_size,
            RED,
        );
    }

    for b in movable_blocks {
        draw_rectangle(
            offset_x + b.0 as f32 * sq_size,
            offset_y + b.1 as f32 * sq_size,
            sq_size,
            sq_size,
            GOLD,
        );
    }

    draw_rectangle(
        offset_x + andi.position.0 as f32 * sq_size,
        offset_y + andi.position.1 as f32 * sq_size,
        sq_size,
        sq_size,
        DARKGREEN,
    );

    draw_text(format!("Steps: {steps}").as_str(), 10., 20., 20., DARKGRAY);
}
}