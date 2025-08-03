use macroquad::prelude::*;
use macroquad::ui::{Skin, hash, root_ui};
mod draw;
mod game_state;
mod level_loader;

use crate::draw::*;
use crate::game_state::DrawGameState;
use crate::game_state::Point;
use crate::level_loader::LevelLoader;

#[macroquad::main("Kanjiban")]
async fn main() {
    let virtual_width = 800.;
    let virtual_height = 600.;

    // Apply a camera that scales everything
    set_camera(&Camera2D {
        target: vec2(virtual_width / 2., virtual_height / 2.),
        zoom: vec2(2. / virtual_width, 2. / virtual_height),
        ..Default::default()
    });

    let mut current_level = 0;

    let initial_loader = LevelLoader::new("levels/level_0.lvl");
    let mut game_state = initial_loader.parse_level().await; // GameState::build_level0();
    let graphical_output = GameBoard::new().await;
    let speed = 0.1;
    let mut last_update = get_time();
    let mut game_over = false; // TODO: move to state

    let no_move: Point = Point { x: 0, y: 0 };
    let up = Point { x: 0, y: -1 };
    let down = Point { x: 0, y: 1 };
    let right = Point { x: 1, y: 0 };
    let left = Point { x: -1, y: 0 };
    let mut selected_level = 0;
    let levels = vec!["0", "1", "gil1"];
    let ui_scale = screen_width() / 800.0; // Base resolution: 800px wide

    loop {
        if current_level != selected_level {
            let ll = LevelLoader::new(&format!(
                "{}{}{}",
                "levels/level_", &levels[selected_level], ".lvl"
            ));
            game_state = ll.parse_level().await;
            current_level = selected_level;
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
        let game_size = 500.;
        if !game_over {
            graphical_output.draw_board(&game_state, 10., 10., game_size as f32);
        } else {
            graphical_output.draw_gameover();
        }
        root_ui().window(
            hash!(),
            vec2((game_size + 20.) * ui_scale + 20., ui_scale * 10.0),
            vec2(ui_scale * 250., ui_scale * 50.),
            |ui| {
                ui.combo_box(hash!(), "Level", &levels, &mut selected_level);
            },
        );
        next_frame().await;
    }
}
