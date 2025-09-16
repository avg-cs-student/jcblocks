//! Holds high-level game logic using components defined elsewhere in the crate.

use std::fmt::{Debug, Display};

use crate::{block::Block, canvas::Canvas};
use rand::{rng, seq::SliceRandom};

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

    /// Generate `n` blocks that are guaranteed to fit within the available playing area.
    pub fn generate_blocks(&self, n: usize) -> Option<Vec<Block>> {
        let mut blocks = Vec::new();
        let mut shadow_canvas = self.canvas.clone();
        for _ in 0..n {
            if let Some(generated_block) = self.naive_generate_block(&mut shadow_canvas) {
                blocks.push(generated_block);
            } else {
                // no more blocks could fit!
                return None;
            }
        }

        blocks.reverse();
        Some(blocks)
    }

    pub fn naive_generate_block(&self, canvas: &mut Canvas) -> Option<Block> {
        let mut all_blocks = [
            Block::rectangle(3, 3),
            Block::rectangle(3, 2),
            Block::rectangle(2, 3),
            Block::rectangle(2, 2),
            Block::rectangle(1, 1),
            Block::tee(),
            Block::line(2),
            Block::line(3),
            Block::line(4),
            Block::line(5),
            Block::elle(3, 3),
            Block::elle(3, 2),
            Block::elle(2, 3),
            Block::elle(2, 2),
            Block::diagonal(2),
            Block::diagonal(3),
            Block::diagonal(4),
        ];

        let mut rng = rng();
        all_blocks.shuffle(&mut rng);
        for block in &mut all_blocks {
            for _ in (0..360).step_by(90) {
                if let Some(playable) = canvas.can_fit(&block) {
                    canvas.add(&playable);
                    return Some(block.to_owned());
                }
                block.rotate_left();
            }
        }

        None
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
