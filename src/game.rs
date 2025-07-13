use std::fmt::{Debug, Display};

use crate::{blocks::Block, canvas::Canvas};
use rand::{random, random_range};

pub struct Game {
    pub canvas: Canvas,
    pub score: usize,
}

impl Game {
    pub fn new() -> Self {
        Self {
            canvas: Canvas::default(),
            score: 0,
        }
    }

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
        let removed = self.canvas.clear_completed_lines();
        self.score += removed;
        Ok(())
    }

    /// Attempt to place a block and increment the game score if necessary.
    pub fn try_place_block(&mut self, block: &Block, row: i32, column: i32) -> &mut Self {
        if let Some(playable) = self.canvas.try_make_playable(block, row, column) {
            self.canvas.add(&playable);
            let removed = self.canvas.clear_completed_lines();
            self.score += removed;
        }

        self
    }
}

impl Default for Game {
    fn default() -> Self {
        Game::new()
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

#[cfg(test)]
mod tests {
    use super::Game;
}
