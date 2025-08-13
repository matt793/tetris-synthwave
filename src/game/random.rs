use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};

use super::piece::Tetromino;

/// Deterministic 7-bag randomizer. Refill occurs when the pool is empty.
pub struct SevenBag {
    rng: StdRng,
    pool: Vec<Tetromino>,
}

impl SevenBag {
    /// If `seed` is None, a default fixed seed is used to keep behavior deterministic for now.
    pub fn new(seed: Option<u64>) -> Self {
        let rng = StdRng::seed_from_u64(seed.unwrap_or(0xC0FFEE_u64));
        let mut this = Self { rng, pool: Vec::with_capacity(7) };
        this.refill();
        this
    }

    fn refill(&mut self) {
        self.pool.clear();
        self.pool.extend_from_slice(&[
            Tetromino::I,
            Tetromino::O,
            Tetromino::T,
            Tetromino::S,
            Tetromino::Z,
            Tetromino::J,
            Tetromino::L,
        ]);
        self.pool.shuffle(&mut self.rng);
    }

    pub fn next(&mut self) -> Tetromino {
        if self.pool.is_empty() {
            self.refill();
        }
        // Safety: pool never empty here
        self.pool.pop().unwrap()
    }

    /// Peek at the next N pieces without consuming them
    pub fn peek(&mut self, count: usize) -> Vec<Tetromino> {
        let mut preview = Vec::with_capacity(count);
        let mut temp_pool = self.pool.clone();
        let mut temp_rng = self.rng.clone();
        
        for _ in 0..count {
            if temp_pool.is_empty() {
                temp_pool.clear();
                temp_pool.extend_from_slice(&[
                    Tetromino::I,
                    Tetromino::O,
                    Tetromino::T,
                    Tetromino::S,
                    Tetromino::Z,
                    Tetromino::J,
                    Tetromino::L,
                ]);
                temp_pool.shuffle(&mut temp_rng);
            }
            preview.push(temp_pool.pop().unwrap());
        }
        
        preview
    }
}
