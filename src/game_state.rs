// kanjiban
// (C) 2025 by JoAn
// Game state structures.

// States a game cell can be in
#[derive(Hash, Clone, Copy, PartialEq, Eq)]
pub enum GameCell {
    Empty,
    Box,
    Player,
    Sink,
    SinkWithBox,
    Unmovable,
}

use std::ops::Add;

use crate::input_control::DIR_NO_MOVE;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
pub struct Player {
    pub position: Point,
    pub direction: Point,
}

pub struct GameState {
    width: i32,
    height: i32,
    cells: Vec<GameCell>,
    joan: Point,
    steps: i16,
    title: String,
    author: String,
}

impl GameState {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width: width as i32,
            height: height as i32,
            cells: vec![GameCell::Empty; (width * height) as usize],
            joan: Point { x: 0, y: 0 },
            steps: 0,
            title: String::from(""),
            author: String::from(""),
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = String::from(title);
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn set_author(&mut self, author: &str) {
        self.author = String::from(author);
    }

    pub fn author(&self) -> &str {
        &self.author
    }

    pub fn steps(&self) -> i16 {
        self.steps
    }

    pub fn get_player_position(&self) -> Point {
        self.joan
    }

    pub fn set_player_position(&mut self, position: &Point) {
        self.joan = *position;
    }

    pub fn get_cell(&self, position: &Point) -> GameCell {
        if position.x >= 0
            && position.x < self.width as i32
            && position.y >= 0
            && position.y < self.height as i32
        {
            self.cells[(position.y * self.width + position.x) as usize]
        } else {
            GameCell::Unmovable
        }
    }

    pub fn set_cell(&mut self, position: &Point, cell: GameCell) {
        if position.x >= 0
            && position.x < self.width as i32
            && position.y >= 0
            && position.y < self.height as i32
        {
            self.cells[(position.y * self.width + position.x) as usize] = cell;
        }
    }

    // next_position: The tile the player likes to move on
    // true if not blocked by wall
    pub fn is_blocked_by_a_wall(&self, pos: &Point) -> bool {
        self.get_cell(&pos) == GameCell::Unmovable
    }

    fn free_cell(&self, pos: &Point) -> bool {
        let cell = self.get_cell(&pos);
        cell == GameCell::Empty || cell == GameCell::Sink
    }

    fn box_cell(&self, pos: &Point) -> bool {
        let cell = self.get_cell(pos);
        cell == GameCell::Box || cell == GameCell::SinkWithBox
    }

    // Return true iff box was successfully moved from old_pos to new_pos;
    // this includes a check whether old_pos really contained a box.
    fn try_move_box(&mut self, old_pos: &Point, new_pos: &Point) -> bool {
        if !self.free_cell(&new_pos) | !self.box_cell(&old_pos) {
            return false;
        }
        let old_cell = self.get_cell(old_pos);
        let new_cell = self.get_cell(new_pos);
        if new_cell == GameCell::Sink {
            self.set_cell(new_pos, GameCell::SinkWithBox);
        } else
        // GameCell::Empty
        {
            self.set_cell(new_pos, GameCell::Box);
        }
        if old_cell == GameCell::SinkWithBox {
            self.set_cell(old_pos, GameCell::Sink);
        } else
        //  GameCell::Box
        {
            self.set_cell(old_pos, GameCell::Empty);
        }
        return true;
    }

    // Return true iff the player can move in the desired direction,
    // and modify the game state. Otherwise return false.
    pub fn perform_move(&mut self, direction: &Point) -> bool {
        if *direction == DIR_NO_MOVE {
            return false;
        }
        let desired_position = *direction + self.get_player_position();
        if self.free_cell(&desired_position) {
            self.set_player_position(&desired_position);
            self.steps += 1;
            return true;
        } else {
            if !self.try_move_box(&desired_position, &(desired_position + *direction)) {
                return false;
            }
            self.set_player_position(&desired_position);
            self.steps += 1;
            return true;
        }
    }

    // checks if the level is solved
    pub fn all_boxes_on_sinks(&self) -> bool {
        !self.cells.iter().any(|c| *c == GameCell::Box)
    }
}
