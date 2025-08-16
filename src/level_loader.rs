// kanjiban
// (C) 2025 by JoAn
// Parser for Sokoban levels
use crate::game_state::{GameCell, GameState, Point};

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
    fn is_level_line(&self, line: &str) -> bool {
        !line.chars().any(|c| !ALLOWED_BOARD_CHARS.contains(c))
    }

    pub async fn parse_level(&self) -> GameState {
        // TODO: better error handling
        let contents = load_string(&self.level_path).await.unwrap();

        let mut current_line_idx = 0;
        assert!(!contents.is_empty());
        let mut parse_state = ParseState::Start;
        let mut y_pos: i32 = 0;
        let mut x_pos: i32 = 0;
        let lines: Vec<&str> = contents.lines().collect();
        let mut width: usize = 0;
        let mut height: usize = 0;
        // peek to get the dimensions of the game board
        for line in &lines {
            if line.is_empty() {
                continue;
            }
            if line.contains(':') {
                break;
            }
            height += 1;
            width = width.max(line.len());
        }
        assert!(height > 0);
        assert!(width > 0);
        let mut result = GameState::new(width as u16, height as u16);
        // actually parse the lines of the game board and
        // the additional info
        while current_line_idx < lines.len() {
            let line = lines[current_line_idx];
            if line.is_empty() {
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
                            '#' => result.set_cell(&pos, GameCell::Unmovable),
                            'p' | '@' => result.set_player_position(&pos),
                            'P' | '+' => {
                                result.set_player_position(&pos);
                                result.set_cell(&pos, GameCell::Sink);
                            }
                            'b' | '$' => {
                                result.set_cell(&pos, GameCell::Box);
                            }
                            'B' | '*' => {
                                result.set_cell(&pos, GameCell::SinkWithBox);
                            }
                            '.' => {
                                result.set_cell(&pos, GameCell::Sink);
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
                    let author_prefix = "Author: ";

                    if line.starts_with(title_prefix) {
                        result.set_title(&line[title_prefix.len()..]);
                    } else if line.starts_with(author_prefix) {
                        result.set_author(&line[author_prefix.len()..]);
                    }
                }
            }
            current_line_idx += 1;
        }
        result
    }
}
