use macroquad::prelude::*;
mod combo_box;
mod game_board;
mod game_state;
mod level_loader;
use crate::combo_box::ComboBox;
use crate::game_board::*;
use crate::game_state::{GameState, Point};
use crate::level_loader::LevelLoader;

// const levels: Vec<&'static str> = vec!["0", "1", "gil1", "thinking_rabbit_1"];
static LEVELS: &'static [&'static str] = &["0", "1", "gil1", "thinking_rabbit_1"];

async fn load_level(level_prefix: &str) -> GameState {
    let ll = LevelLoader::new(&format!("{}{}{}", "levels/level_", level_prefix, ".lvl"));
    ll.parse_level().await
}
#[macroquad::main("Kanjiban")]
async fn main() {
    let virtual_width = 800.;
    let virtual_height = 600.;

    // Apply a camera that scales everything
    let camera = Camera2D {
        target: vec2(virtual_width / 2., virtual_height / 2.),
        zoom: vec2(2. / virtual_width, 2. / virtual_height),
        ..Default::default()
    };
    set_camera(&camera);

    let mut game_state = load_level("0").await;
    let game_board = GameBoard::new(10., 10., 500.).await;
    let mut combo = ComboBox::new(&camera, 500. + 20.0, 20., 200.0, &LEVELS);
    let speed = 0.1;
    let mut last_update = get_time();
    let mut game_over = false; // TODO: move to state

    let no_move: Point = Point { x: 0, y: 0 };
    let up = Point { x: 0, y: -1 };
    let down = Point { x: 0, y: 1 };
    let right = Point { x: 1, y: 0 };
    let left = Point { x: -1, y: 0 };

    loop {
        if let Some(selected) = combo.update() {
            game_state = load_level(&LEVELS[selected]).await;
            game_over = false;
        }
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
            game_board.draw_board(&game_state);
        } else {
            game_board.draw_win(&game_state);
        }
        combo.draw();

        next_frame().await;
    }
}
