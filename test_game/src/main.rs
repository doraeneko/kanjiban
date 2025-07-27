use macroquad::prelude::*;
use std::collections::LinkedList;

mod game_state;
pub use crate::game_state::board::Point;
pub use crate::game_state::board::SQUARES;
pub use crate::game_state::board::Player;
pub use crate::game_state::board::new_level;
pub use crate::game_state::board::is_blocked_by_a_wall;
pub use crate::game_state::board::box_is_blocked;

mod render;
pub use crate::render::draw::draw_game;

fn all_boxes_on_sinks(movable_blocks: &LinkedList<Point>, sinks: &LinkedList<Point>) -> bool {
    for s in sinks {
        let mut sink_found = false;
        for b in movable_blocks {
            if b == s {
                sink_found = true;
            }
        }
        if !sink_found {
            return false;
        }
    }
    return true;
}

#[macroquad::main("Kanjiban")]
async fn main() {
    let mut andi = Player {
        position: (10, 4),
        dir: (1, 0),
    };

    let mut level = new_level();

    let mut steps = 0;
    let speed = 0.1;
    let mut last_update = get_time();
    let mut game_over = false;

    let up = (0, -1);
    let down = (0, 1);
    let right = (1, 0);
    let left = (-1, 0);

    loop {
        if !game_over {
            andi.dir = (0, 0);
            if is_key_down(KeyCode::Right) {
                andi.dir = right;
            } else if is_key_down(KeyCode::Left) {
                andi.dir = left;
            } else if is_key_down(KeyCode::Up) {
                andi.dir = up;
            } else if is_key_down(KeyCode::Down) {
                andi.dir = down;
            }

            if get_time() - last_update > speed {
                last_update = get_time();
                // player likes to move on this tile:
                let next_position: Point =
                    (andi.position.0 + andi.dir.0, andi.position.1 + andi.dir.1);
                let mut player_can_move = true;
                if is_blocked_by_a_wall(&level, &next_position) {
                    player_can_move = false;
                }
                if box_is_blocked(&mut level, &andi, &next_position) {
                    player_can_move = false;
                }

                if player_can_move && next_position != andi.position {
                    andi.position = next_position;
                    steps += 1;
                    game_over = all_boxes_on_sinks(&mut level.movable_blocks, &mut level.sinks);
                }
            }
        }
        if !game_over {
            draw_game(
                &mut level,
                &andi,
                steps,
            );
        } else {
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
        next_frame().await;
    }
}
