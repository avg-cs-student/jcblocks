use std::fmt::{Debug, Display};

use crate::{block::Block, canvas::Canvas};
use rand::{random, random_range};

const POINTS_PER_LINE_CLEAR: usize = 50;

pub struct Game {
    pub canvas: Canvas,
    pub score: usize,
}

impl Game {
    pub fn reset(&mut self) -> &mut Self {
        self.canvas.clear_all();
        self.score = 0;
        self
    }

    pub fn generate_blocks(&self, n: usize) -> Vec<Block> {
        let mut blocks = Vec::new();
        for _ in 0..n {
            let mut block: Block = random();
            let rotations: usize = random_range(0..360) / 90;
            for _ in 0..=rotations {
                block.rotate_left();
            }
            blocks.push(block);
        }

        blocks
    }

    pub fn is_playable(&self, block: &Block) -> bool {
        self.canvas.can_fit(block)
    }

    pub fn maybe_place_block(&mut self, block: &Block, row: i32, column: i32) -> Result<(), &str> {
        let Some(playable) = self.canvas.try_make_playable(block, row, column) else {
            return Err("Unable to place block.");
        };

        self.canvas.add(&playable);
        let lines_cleared = self.canvas.clear_completed_lines();
        self.update_score(lines_cleared);

        Ok(())
    }

    fn update_score(&mut self, lines_cleared: usize) -> &mut Self {
        self.score += lines_cleared * POINTS_PER_LINE_CLEAR;
        self
    }
}

impl Default for Game {
    fn default() -> Self {
        Self {
            canvas: Canvas::default(),
            score: 0,
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.canvas.fmt(f)
    }
}

impl Debug for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.canvas.fmt(f)
    }
}
