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
    width: usize,
    height: usize,
    cells: Vec<GameCell>,
    pub unmovable_blocks: LinkedList<Point>,
    pub movable_blocks: LinkedList<Point>,
    pub sinks: LinkedList<Point>,
    joan: Player,
    steps: i16,
    title: String,
    author: String,
}

impl GameState {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![GameCell::Empty; width * height],
            unmovable_blocks: LinkedList::new(),
            movable_blocks: LinkedList::new(),
            sinks: LinkedList::new(),
            joan: Player {
                position: Point { x: 0, y: 0 },
                direction: Point { x: 0, y: 0 },
            },
            steps: 0,
            title: String::from(""),
            author: String::from(""),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
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

    pub fn inc_steps(&mut self) {
        self.steps += 1;
    }

    pub fn steps(&self) -> i16 {
        self.steps
    }

    pub fn set_direction(&mut self, direction: Point) {
        self.joan.direction = direction;
    }

    pub fn get_direction(&self) -> Point {
        self.joan.direction
    }

    pub fn get_player_position(&self) -> Point {
        self.joan.position
    }

    pub fn set_player_position(&mut self, position: Point) {
        self.joan.position = position;
    }

    // next_position: The tile the player likes to move on
    // true if not blocked by wall
    pub fn is_blocked_by_a_wall(&self, next_position: &Point) -> bool {
        for b in &self.unmovable_blocks {
            if *b == *next_position {
                // the tile is blocked
                return true;
            }
        }
        false
    }

    // checks if next position is a tile with a box and if this box can be moved
    // FIXME(SRP): also moves box
    pub fn box_is_blocked(self: &mut GameState, next_position: &Point) -> bool {
        if self.movable_blocks.contains(next_position) {
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
        false
    }

    // check if a block can be moved to next_block_position
    fn is_block_movable(&self, next_block_position: Point) -> bool {
        let block_contained = |b: &Point| *b == next_block_position;
        !self.unmovable_blocks.iter().any(block_contained)
            && !self.movable_blocks.iter().any(block_contained)
    }

    pub fn is_target(&self, position: &Point) -> bool {
        self.sinks.contains(position)
    }

    // checks if the level is solved
    pub fn all_boxes_on_sinks(&self) -> bool {
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
        true
    }
}
