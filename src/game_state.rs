// kanjiban
// (C) 2025 by JoAn
// Game state structures.

use std::collections::LinkedList;

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
    pub width: u16,
    pub height: u16,
    pub 
    unmovable_blocks: LinkedList<Point>,
    pub movable_blocks: LinkedList<Point>,
    pub sinks: LinkedList<Point>,
    joan: Player,
    steps: i16,
    title: String,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            unmovable_blocks: LinkedList::new(),
            movable_blocks: LinkedList::new(),
            sinks: LinkedList::new(),
            joan: Player {
                position: Point { x: 0, y: 0 },
                direction: Point { x: 0, y: 0 },
            },
            steps: 0,
            title: String::from(""),
        }
    }

    pub fn set_title(self: &mut Self, title: &str) {
        self.title = String::from(title);
    }

    pub fn get_title(self: &Self) -> &str {
        return &self.title;
    }

    pub fn inc_steps(self: &mut Self) {
        self.steps += 1;
    }

    pub fn steps(self: &Self) -> i16 {
        return self.steps;
    }

    pub fn set_direction(self: &mut Self, direction: Point) {
        self.joan.direction = direction;
    }

    pub fn get_direction(self: &Self) -> Point {
        return self.joan.direction;
    }

    pub fn get_player_position(self: &Self) -> Point {
        return self.joan.position;
    }

    pub fn set_player_position(self: &mut Self, position: Point) {
        self.joan.position = position;
    }

    // next_position: The tile the player likes to move on
    // true if not blocked by wall
    pub fn is_blocked_by_a_wall(self: &Self, next_position: &Point) -> bool {
        for b in &self.unmovable_blocks {
            if *b == *next_position {
                // the tile is blocked
                return true;
            }
        }
        return false;
    }

    // checks if next position is a tile with a box and if this box can be moved
    // FIXME(SRP): also moves box
    pub fn box_is_blocked(self: &mut GameState, next_position: &Point) -> bool {
        if self.movable_blocks.contains(&next_position) {
            let next_block_position = Point {
                x: next_position.x + self.joan.direction.x,
                y: next_position.y + self.joan.direction.y,
            };
            if self.is_block_movable(next_block_position) {
                //movable_blocks.iter().find(|x: Point | *x == next_position)
                for b in &mut self.movable_blocks {
                    if *b == *next_position {
                        let next_block_position = Point {
                            x: b.x + self.joan.direction.x,
                            y: b.y + self.joan.direction.y,
                        };
                        *b = next_block_position; // move the box
                    }
                }
            } else {
                return true;
            }
        }
        return false;
    }

    // check if a block can be moved to next_block_position
    fn is_block_movable(self: &Self, next_block_position: Point) -> bool {
        let block_contained = |b: &Point| *b == next_block_position;
        !self.unmovable_blocks.iter().any(block_contained)
            && !self.movable_blocks.iter().any(block_contained)
    }

    pub fn is_target(self: &Self, position: &Point) -> bool {
        return self.sinks.contains(&position);
    }

    // checks if the level is solved
    pub fn all_boxes_on_sinks(self: &Self) -> bool {
        for s in &self.sinks {
            let mut sink_found = false;
            for b in &self.movable_blocks {
                if *b == *s {
                    sink_found = true;
                }
            }
            if !sink_found {
                return false;
            }
        }
        return true;
    }
}
