use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng, thread_rng, Rng};

use super::piece::Tetromino;

/// Deterministic 7-bag randomizer. Refill occurs when the pool is empty.
pub struct SevenBag {
    rng: StdRng,
    pool: Vec<Tetromino>,
}

impl SevenBag {
    /// If `seed` is None, a random seed is generated for true randomization.
    pub fn new(seed: Option<u64>) -> Self {
        let rng = match seed {
            Some(s) => StdRng::seed_from_u64(s),
            None => {
                // Generate a truly random seed
                let random_seed = thread_rng().gen::<u64>();
                StdRng::seed_from_u64(random_seed)
            }
        };
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
