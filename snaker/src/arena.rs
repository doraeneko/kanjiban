// Snaker
// (C) 2025, part of Kanjiban by JoAn.
// Game arena.

use rand::prelude::SliceRandom;
use rand::thread_rng;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CellContent {
    Empty,
    Snake,
    Food,
}

#[derive(Clone, Copy)]
pub struct ArenaPosition {
    pub x: usize,
    pub y: usize,
}

pub struct Arena {
    pub width: usize,
    pub height: usize,
    cells: Vec<CellContent>,
}

impl Arena {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![CellContent::Empty; width * height],
        }
    }

    pub fn food_left(&self) -> usize {
        let total_cells = self.width * self.height;
        return (0..total_cells)
            .filter(|&idx| self.cells[idx] == CellContent::Food)
            .count();
    }

    pub fn distribute_food(&mut self, how_much: usize) {
        let total_cells = self.width * self.height;

        // Get a list of all cell indices
        let mut indices: Vec<usize> = (0..total_cells)
            .filter(|&idx| self.cells[idx] == CellContent::Empty)
            .collect();

        assert!(
            how_much <= indices.len(),
            "Can't place more food than cells + initial snake"
        );

        // Shuffle them
        indices.shuffle(&mut thread_rng());

        // Pick the first n indices and set them to Food
        for &idx in indices.iter().take(how_much) {
            self.cells[idx] = CellContent::Food;
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: CellContent) {
        if x < self.width && y < self.height {
            self.cells[y * self.width + x] = value;
        }
    }

    pub fn set_pos(&mut self, pos: &ArenaPosition, value: CellContent) {
        return self.set(pos.x, pos.y, value);
    }

    pub fn get(&self, x: usize, y: usize) -> CellContent {
        if x < self.width && y < self.height {
            self.cells[y * self.width + x]
        } else {
            CellContent::Empty
        }
    }

    pub fn get_pos(&self, pos: &ArenaPosition) -> CellContent {
        return self.get(pos.x, pos.y);
    }
}
