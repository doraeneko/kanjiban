use macroquad::prelude::*;

mod draw;
mod game_state;
use crate::draw::*;
use crate::game_state::DrawGameState;
pub use crate::game_state::GameState;
pub use crate::game_state::Point;

#[macroquad::main("Kanjiban")]
async fn main() {
    let mut game_state = GameState::build_level0();
    let graphical_output = GameBoard::new(game_state.width, game_state.height);
    let speed = 0.1;
    let mut last_update = get_time();
    let mut game_over = false; // TODO: move to state

    let no_move: Point = Point { x: 0, y: 0 };
    let up = Point { x: 0, y: -1 };
    let down = Point { x: 0, y: 1 };
    let right = Point { x: 1, y: 0 };
    let left = Point { x: -1, y: 0 };

    loop {
        if !game_over {
            let mut direction = no_move;
            if is_key_down(KeyCode::Right) {
                direction = right;
            } else if is_key_down(KeyCode::Left) {
                direction = left;
            } else if is_key_down(KeyCode::Up) {
                direction = up;
            } else if is_key_down(KeyCode::Down) {
                direction = down;
            }
            game_state.set_direction(direction);

            if get_time() - last_update > speed {
                last_update = get_time();
                // player likes to move on this tile:
                let next_position: Point = game_state.get_player_position() + direction;
                let mut player_can_move = true;
                if game_state.is_blocked_by_a_wall(&next_position) {
                    player_can_move = false;
                }
                if game_state.box_is_blocked(&next_position) {
                    player_can_move = false;
                }

                if player_can_move && next_position != game_state.get_player_position() {
                    game_state.set_player_position(next_position);
                    game_state.inc_steps();
                    game_over = game_state.all_boxes_on_sinks();
                }
            }
        }
        if !game_over {
            graphical_output.draw_board(&game_state);
        } else {
            graphical_output.draw_gameover();
        }
        next_frame().await;
    }
}
