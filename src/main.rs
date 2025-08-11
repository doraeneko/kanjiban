use macroquad::prelude::*;
mod game_board;
mod game_state;
mod input_control;
mod level_chooser;
mod level_loader;
mod macroquad_helpers;
use crate::game_board::*;
use crate::game_state::{GameState, Point};
use crate::input_control::{DIR_NO_MOVE, InputControl};
use crate::level_chooser::LevelChooser;
use crate::level_loader::LevelLoader;

fn window_conf() -> Conf {
    Conf {
        window_title: "Kanjiban".to_owned(),
        window_width: 1280,
        window_height: 720,
        fullscreen: false,
        ..Default::default()
    }
}

static LEVELS: &'static [&'static str] = &["0", "1", "2", "3", "4", "5"];

async fn load_level(level_prefix: &str) -> GameState {
    let ll = LevelLoader::new(&format!("{}{}{}", "levels/level_", level_prefix, ".lvl"));
    ll.parse_level().await
}

fn draw_status_bar(game_state: &GameState) {
    let steps = game_state.steps();
    let start_x = 35.;
    let start_y = 35.;
    let mut text_height = start_y;
    draw_text(
        format!("Steps: {steps}").as_str(),
        start_x,
        text_height,
        30.,
        DARKGRAY,
    );
    text_height = 20.;
    draw_text(
        game_state.get_title(),
        start_x + 250.0,
        text_height,
        20.,
        BLUE,
    );
    text_height += 20.;
    draw_text(
        game_state.get_author(),
        start_x + 250.0,
        text_height,
        20.,
        BLUE,
    );
}

#[macroquad::main(window_conf)]
async fn main() {
    let virtual_width = 1280.;
    let virtual_height = 720.;
    // Apply a camera that scales everything
    let camera = Camera2D {
        target: vec2(virtual_width / 2., virtual_height / 2.),
        zoom: vec2(2. / virtual_width, 2. / virtual_height),
        ..Default::default()
    };
    set_camera(&camera);
    let game_board = GameBoard::new(2., 50., 1270., 660.).await;
    let mut level_chooser = LevelChooser::new(&camera, 850.0, 2., 200.0, LEVELS);
    let mut input_control = InputControl::new(&camera);
    let speed: f64 = 0.1;
    let mut last_update = get_time();
    let mut game_over = false; // TODO: move to state
    let mut game_state = load_level("0").await;

    loop {
        if let Some(selected) = level_chooser.update() {
            game_state = load_level(LEVELS[selected]).await;
            game_over = false;
        }
        let direction = input_control.get_direction().await;
        game_state.set_direction(direction);

        if get_time() - last_update > speed {
            last_update = get_time();
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

            game_state.set_direction(DIR_NO_MOVE);
        }

        if !game_over {
            game_board.draw_board(&game_state);
        } else {
            game_board.draw_win(&game_state);
        }
        draw_status_bar(&game_state);
        level_chooser.draw();

        next_frame().await;
    }
}
