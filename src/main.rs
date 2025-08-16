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
use crate::macroquad_helpers::FontProvider;

fn window_conf() -> Conf {
    Conf {
        window_title: "Kanjiban".to_owned(),
        window_width: 1280,
        window_height: 720,
        fullscreen: false,
        ..Default::default()
    }
}

static LEVELS: &[&str] = &["0", "1", "2", "3", "4", "5"];

async fn load_level(level_prefix: &str) -> GameState {
    let ll = LevelLoader::new(&format!("{}{}{}", "levels/level_", level_prefix, ".lvl"));
    ll.parse_level().await
}

fn draw_status_bar(game_state: &GameState, fonts: &FontProvider) {
    let steps = game_state.steps();
    let start_x = 35.;
    let start_y = 65.;
    let mut text_height = start_y;
    let gray_text_params = TextParams {
        font: Some(fonts.font()),
        font_size: 50,
        color: DARKGRAY,
        ..Default::default()
    };
    let blue_text_params = TextParams {
        font: Some(fonts.font()),
        font_size: 30,
        color: BLUE,
        ..Default::default()
    };

    draw_text_ex(
        format!("Steps: {steps}").as_str(),
        start_x,
        text_height,
        gray_text_params,
    );
    text_height = 30.;
    draw_text_ex(
        game_state.title(),
        start_x + 250.0,
        text_height,
        blue_text_params.clone(),
    );
    text_height += 30.;
    draw_text_ex(
        game_state.author(),
        start_x + 250.0,
        text_height,
        blue_text_params.clone(),
    );
}

#[macroquad::main(window_conf)]
async fn main() {
    let fonts = FontProvider::new().await;
    let virtual_width = 1280.;
    let virtual_height = 720.;
    // Apply a camera that scales everything
    let camera = Camera2D {
        target: vec2(virtual_width / 2., virtual_height / 2.),
        zoom: vec2(2. / virtual_width, 2. / virtual_height),
        ..Default::default()
    };
    set_camera(&camera);
    let game_board = GameBoard::new(2., 110., 1270., 600.).await;
    let mut level_chooser = LevelChooser::new(&camera, 920.0, 2., 350.0, LEVELS, &fonts);
    let mut input_control = InputControl::new();
    let speed: f64 = 0.25;
    let mut last_update = get_time();
    let mut game_over = false; // TODO: move to state
    let mut game_state = load_level("0").await;
    let mut desired_direction: Point = DIR_NO_MOVE;

    loop {
        if let Some(selected) = level_chooser.update() {
            game_state = load_level(LEVELS[selected]).await;
            game_over = false;
        }
        let current_direction = input_control.get_direction();
        if current_direction != DIR_NO_MOVE {
            desired_direction = current_direction;
        }
        if get_time() - last_update > speed {
            last_update = get_time();
            // player likes to move on this tile:
            if game_state.perform_move(&desired_direction) {
                game_over = game_state.all_boxes_on_sinks();
            }
            desired_direction = DIR_NO_MOVE;
        }

        if !game_over {
            game_board.draw_board(&game_state);
        } else {
            game_board.draw_win(&game_state);
        }
        draw_status_bar(&game_state, &fonts);
        level_chooser.draw();

        next_frame().await;
    }
}
