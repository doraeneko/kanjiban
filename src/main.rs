use macroquad::prelude::*;
mod combo_box;
mod dir_pad;
mod game_board;
mod game_state;
mod level_loader;
mod macroquad_helpers;
use crate::combo_box::ComboBox;
use crate::dir_pad::{DIR_NO_MOVE, DirPad};
use crate::game_board::*;
use crate::game_state::{GameState, Point};
use crate::level_loader::LevelLoader;

fn window_conf() -> Conf {
    Conf {
        window_title: "Kanjiban".to_owned(),
        window_width: 720,
        window_height: 1280,
        fullscreen: false,
        ..Default::default()
    }
}

static LEVELS: &'static [&'static str] = &["0", "1", "gil1", "thinking_rabbit_1"];

async fn load_level(level_prefix: &str) -> GameState {
    let ll = LevelLoader::new(&format!("{}{}{}", "levels/level_", level_prefix, ".lvl"));
    ll.parse_level().await
}
#[macroquad::main(window_conf)]
async fn main() {
    let virtual_width = 720.;
    let virtual_height = 1280.;
    // Apply a camera that scales everything
    let camera = Camera2D {
        target: vec2(virtual_width / 2., virtual_height / 2.),
        zoom: vec2(2. / virtual_width, 2. / virtual_height),
        ..Default::default()
    };
    set_camera(&camera);
    let game_board = GameBoard::new(5., 70., 700.).await;
    let mut combo = ComboBox::new(&camera, 50.0, 10., 500.0, &LEVELS);
    let dir_pad = DirPad::new(&camera, 360., 1000.0, 500.);
    let speed: f64 = 0.25;
    let mut last_update = get_time();
    let mut game_over = false; // TODO: move to state
    let mut push_requested = false;

    let mut game_state = load_level("0").await;

    loop {
        if let Some(selected) = combo.update() {
            game_state = load_level(&LEVELS[selected]).await;
            game_over = false;
        }
        if !push_requested {
            let direction = dir_pad.get_direction().await;
            if direction != DIR_NO_MOVE {
                game_state.set_direction(direction);
                push_requested = true;
            }
        }
        if !game_over {
            if get_time() - last_update > speed {
                last_update = get_time();
                if push_requested {
                    push_requested = false;
                    // player likes to move on this tile:
                    let next_position: Point =
                        game_state.get_player_position() + game_state.get_direction();
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
                game_state.set_direction(DIR_NO_MOVE);
            }
        }
        if !game_over {
            game_board.draw_board(&game_state);
        } else {
            game_board.draw_win(&game_state);
        }
        combo.draw();
        dir_pad.draw();

        next_frame().await;
    }
}
