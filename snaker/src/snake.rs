// Snaker
// (C) 2025, part of Kanjiban by JoAn.
// Snake abstraction.

use crate::arena::*;
use std::collections::VecDeque;

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

pub struct Snake {
    pub body: VecDeque<ArenaPosition>,
    pub direction: Direction,
}

impl Snake {
    pub fn new(arena: &mut Arena) -> Self {
        let midpoint = ArenaPosition {
            x: arena.width / 2,
            y: arena.height / 2,
        };
        return Self {
            body: VecDeque::from([midpoint, midpoint]),
            direction: Direction::Up,
        };
        // The snake is always at least two elements long.
    }

    fn is_covered_by_snake(&self, pos: &ArenaPosition) -> bool {
        for snake_pos in &self.body {
            if snake_pos.x == pos.x && snake_pos.y == pos.y {
                return true;
            }
        }
        return false;
    }

    pub fn set_direction(&mut self, new_direction: Direction) {
        self.direction = new_direction;
    }

    // returns true iff the snake is still collision-free and does
    // not leave the game arena. Updates the arena.
    pub fn next(&mut self, arena: &mut Arena) -> bool {
        // obtain the head/tail element of the snake

        if self.body.len() < 2 {
            return false;
        }

        let mut head = self.body.pop_front().unwrap();
        let tail = self.body.pop_back().unwrap();
        self.body.push_front(head); // idiotic, but to avoid borrow check madness

        // compute next head position
        match self.direction {
            Direction::Up => {
                if head.y == 0 {
                    return false;
                }
                head.y = head.y - 1;
            }
            Direction::Down => {
                if head.y == arena.height - 1 {
                    return false;
                }
                head.y = head.y + 1;
            }
            Direction::Left => {
                if head.x == 0 {
                    return false;
                }
                head.x = head.x - 1;
            }
            Direction::Right => {
                if head.x == arena.width - 1 {
                    return false;
                }
                head.x = head.x + 1;
            }
        }
        if self.is_covered_by_snake(&head) {
            return false; // collision
        }
        self.body.push_front(head);
        arena.set_pos(&tail, CellContent::Empty);
        // set tail cell to empty. It is however
        //  possible that the multiple snake segments are at the tail.
        // Therefore we (re)set the complete snake body later again.

        if arena.get_pos(&head) == CellContent::Food {
            arena.set_pos(&head, CellContent::Empty);
            self.body.push_back(tail); // prolong the snake
        }

        for snake_pos in &self.body {
            arena.set_pos(&snake_pos, CellContent::Snake);
        }

        return true;
    }
}
