// kanjiban
// (C) 2025 by JoAn
// Game state structures.

use std::collections::LinkedList;

pub const SQUARES: usize = 16;
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
    pub width: usize,
    pub height: usize,
    pub unmovable_blocks: LinkedList<Point>,
    pub movable_blocks: LinkedList<Point>,
    pub sinks: LinkedList<Point>,
    andi: Player,
    steps: i16,
}

// Graphical outputs should implement this interface
pub trait DrawGameState {
    fn draw_board(&self, game_state: &GameState);
    fn draw_gameover(&self);
}

impl GameState {
    // taken from wikipedia
    pub fn build_level0() -> Self {
        let mut state: GameState = GameState {
            width: SQUARES,
            height: SQUARES,
            unmovable_blocks: LinkedList::new(),
            movable_blocks: LinkedList::new(),
            sinks: LinkedList::new(),
            andi: Player {
                position: Point { x: 10, y: 4 },
                direction: Point { x: 1, y: 0 },
            },
            steps: 0,
        };

        // place none movable blocks
        for i in 0..5 {
            state.unmovable_blocks.push_front(Point { x: 4, y: i });
            state.unmovable_blocks.push_front(Point { x: 12, y: i });
        }
        for i in 4..13 {
            state.unmovable_blocks.push_front(Point { x: i, y: 0 });
            state.unmovable_blocks.push_front(Point { x: i, y: 5 });
        }
        for i in 4..7 {
            state.unmovable_blocks.push_front(Point { x: i, y: 1 });
        }
        for i in 9..12 {
            state.unmovable_blocks.push_front(Point { x: i, y: 1 });
        }
        // add some none-movable blocks in the middle
        state.unmovable_blocks.push_front(Point { x: 9, y: 3 });
        state.unmovable_blocks.push_front(Point { x: 9, y: 4 });
        state.unmovable_blocks.push_front(Point { x: 6, y: 3 });

        // add movable boxes
        state.movable_blocks.push_front(Point { x: 10, y: 2 });
        state.movable_blocks.push_front(Point { x: 10, y: 3 });

        // add sinks
        state.sinks.push_front(Point { x: 8, y: 4 });
        state.sinks.push_front(Point { x: 6, y: 4 });
        return state;
    }

    pub fn inc_steps(self: &mut Self) {
        self.steps += 1;
    }

    pub fn steps(self: &Self) -> i16 {
        return self.steps;
    }

    pub fn set_direction(self: &mut Self, direction: Point) {
        self.andi.direction = direction;
    }

    pub fn get_direction(self: &Self) -> Point {
        return self.andi.direction;
    }

    pub fn get_player_position(self: &Self) -> Point {
        return self.andi.position;
    }

    pub fn set_player_position(self: &mut Self, next_position: Point) {
        self.andi.position = next_position;
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
                x: next_position.x + self.andi.direction.x,
                y: next_position.y + self.andi.direction.y,
            };
            if self.is_block_movable(next_block_position) {
                //movable_blocks.iter().find(|x: Point | *x == next_position)
                for b in &mut self.movable_blocks {
                    if *b == *next_position {
                        let next_block_position = Point {
                            x: b.x + self.andi.direction.x,
                            y: b.y + self.andi.direction.y,
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
