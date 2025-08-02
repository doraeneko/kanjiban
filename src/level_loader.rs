// kanjiban
// (C) 2025 by JoAn
// Parser for Sokoban levels
use crate::game_state::{GameState, Point};
use macroquad::prelude::*;

const ALLOWED_BOARD_CHARS: &str = "#pPbB._ -.$*+@";

enum ParseState {
    Start,
    ReadGameBoard,
}
pub struct LevelLoader {
    level_path: String,
}

impl LevelLoader {
    pub fn new(level_path: &str) -> Self {
        Self {
            level_path: String::from(level_path),
        }
    }

    // returns true iff only characters for describing board content are contained in line.
    fn is_level_line(self: &Self, line: &str) -> bool {
        return !line.chars().any(|c| !ALLOWED_BOARD_CHARS.contains(c));
    }

    pub async fn parse_level(self: &Self) -> GameState {
        let contents = load_string(&self.level_path).await.unwrap();

        let mut current_line_idx = 0;
        assert!(!contents.is_empty());
        let mut result = GameState::new();
        let mut parse_state = ParseState::Start;
        let mut y_pos: i32 = 0;
        let mut x_pos: i32 = 0;
        let lines: Vec<&str> = contents.lines().collect();
        while current_line_idx < lines.len() {
            let line = lines[current_line_idx];
            if line.len() == 0 {
                current_line_idx += 1;
                continue;
            }
            match parse_state {
                ParseState::Start => {
                    if !self.is_level_line(line) {
                        parse_state = ParseState::ReadGameBoard;
                        continue; // no increase of current_line_idx
                    }
                    for c in line.chars() {
                        let pos = Point { x: x_pos, y: y_pos };
                        match c {
                            '#' => result.unmovable_blocks.push_back(pos),
                            'p' | '@' => result.set_player_position(pos),
                            'P' | '+' => {
                                result.set_player_position(pos);
                                result.sinks.push_back(pos);
                            }
                            'b' | '$' => {
                                result.movable_blocks.push_back(pos);
                            }
                            'B' | '*' => {
                                result.movable_blocks.push_back(pos);
                                result.sinks.push_back(pos);
                            }
                            '.' => {
                                result.sinks.push_back(pos);
                            }
                            _ => {} // floor
                        }
                        x_pos += 1;
                    }
                    y_pos += 1;
                    x_pos = 0;
                }
                ParseState::ReadGameBoard => {
                    let title_prefix = "Title: ";
                    if line.starts_with(title_prefix) {
                        result.set_title(&line[title_prefix.len()..]);
                    } // ignore other key/value pairs for now.
                }
            }
            current_line_idx += 1;
        }
        result
    }
}
