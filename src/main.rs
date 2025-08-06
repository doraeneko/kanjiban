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
    let text_width_offset = 850.;
    let text_height_offset = 600.0;
    let mut text_height = text_height_offset;
    draw_text(
        format!("Steps: {steps}").as_str(),
        text_width_offset,
        text_height,
        30.,
        DARKGRAY,
    );
    let line_chunks = game_state.get_title().chars().collect::<Vec<_>>();
    let title_lines = line_chunks.chunks(40);
    text_height += 35.;
    for line in title_lines {
        let output: String = line.iter().collect();
        draw_text(&output, text_width_offset, text_height, 20., BLUE);
        text_height += 25.;
    }
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
    let game_board = GameBoard::new(2., 2., 800.).await;
    let mut combo = ComboBox::new(&camera, 850.0, 2., 200.0, &LEVELS);
    let dir_pad = DirPad::new(&camera, 1050., 350.0, 400.);
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
        draw_status_bar(&game_state);
        dir_pad.draw();
        combo.draw();

        next_frame().await;
    }
}
