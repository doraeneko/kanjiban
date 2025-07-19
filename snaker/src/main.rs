// Snaker
// (C) 2025, part of Kanjiban by JoAn.
// Main logic.

use macroquad::prelude::*;
mod arena;
mod draw;
mod snake;
use crate::arena::*;
use crate::draw::*;
use crate::snake::*;

const UPDATE_RATE_IN_SEC: f32 = 0.25;
const ARENA_X: usize = 20;
const ARENA_Y: usize = 20;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Running,
    Lost,
    Won,
}

fn reset(arena: &mut Arena, snake: &mut Snake) {
    *arena = Arena::new(ARENA_X, ARENA_Y);
    *snake = Snake::new(arena);
    arena.distribute_food(20);
}

#[macroquad::main("Snaker")]
async fn main() {
    let mut arena: Arena = Arena::new(ARENA_X, ARENA_Y);
    let mut snake: Snake = Snake::new(&mut arena);
    reset(&mut arena, &mut snake);
    let mut elapsed = 0.0;
    let mut game_state: GameState = GameState::Running;
    loop {
        let dt = get_frame_time();
        elapsed += dt;
        if elapsed >= UPDATE_RATE_IN_SEC {
            elapsed = 0.0;
            if game_state == GameState::Running {
                if !snake.next(&mut arena) {
                    game_state = GameState::Lost;
                } else if arena.food_left() == 0 {
                    game_state = GameState::Won;
                }
            }
        }
        // input loop
        if is_key_down(KeyCode::Right) {
            snake.set_direction(Direction::Right);
        } else if is_key_down(KeyCode::Left) {
            snake.set_direction(Direction::Left);
        } else if is_key_down(KeyCode::Up) {
            snake.set_direction(Direction::Up);
        } else if is_key_down(KeyCode::Down) {
            snake.set_direction(Direction::Down);
        } else if is_key_down(KeyCode::Enter) {
            reset(&mut arena, &mut snake);
            game_state = GameState::Running;
        }
        match game_state {
            GameState::Running => {
                clear_background(LIGHTGRAY);
                draw_arena(&arena);
            }
            GameState::Lost => {
                game_over(false);
            }
            GameState::Won => {
                game_over(true);
            }
        }
        next_frame().await;
    }
}
