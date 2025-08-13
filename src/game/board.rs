use super::piece::{ActivePiece, Tetromino, blocks};
use super::{BOARD_W, BOARD_H};

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub t: Tetromino,
    pub _power: bool, // reserved for later mechanics
}

#[derive(Debug, Clone)]
pub struct Board {
    pub w: i16,
    pub h: i16,
    cells: Vec<Option<Cell>>,
}

impl Board {
    pub fn new() -> Self {
        let w = BOARD_W;
        let h = BOARD_H;
        Self {
            w,
            h,
            cells: vec![None; (w * h) as usize],
        }
    }

    #[inline]
    fn idx(&self, x: i16, y: i16) -> usize {
        (y as usize) * (self.w as usize) + (x as usize)
    }

    #[inline]
    pub fn in_bounds(&self, x: i16, y: i16) -> bool {
        x >= 0 && x < self.w && y >= 0 && y < self.h
    }

    pub fn get(&self, x: i16, y: i16) -> Option<Cell> {
        if !self.in_bounds(x, y) {
            return None;
        }
        self.cells[self.idx(x, y)]
    }

    pub fn set(&mut self, x: i16, y: i16, val: Option<Cell>) {
        if self.in_bounds(x, y) {
            let i = self.idx(x, y);
            self.cells[i] = val;
        }
    }

    pub fn clear(&mut self) {
        self.cells.fill(None);
    }

    pub fn collides(&self, p: &ActivePiece) -> bool {
        for (ox, oy) in blocks(p.t, p.rot) {
            let x = p.x + ox;
            let y = p.y + oy;
            if !self.in_bounds(x, y) {
                return true;
            }
            if self.get(x, y).is_some() {
                return true;
            }
        }
        false
    }

    pub fn lock_piece(&mut self, p: &ActivePiece) {
        for (ox, oy) in blocks(p.t, p.rot) {
            let x = p.x + ox;
            let y = p.y + oy;
            if self.in_bounds(x, y) {
                self.set(x, y, Some(Cell { t: p.t, _power: false }));
            }
        }
    }

    /// Clears all full lines. Returns number of cleared lines.
    pub fn clear_full_lines(&mut self) -> u32 {
        let mut write_y = self.h - 1;
        let mut cleared = 0u32;

        for read_y in (0..self.h).rev() {
            let mut full = true;
            for x in 0..self.w {
                if self.get(x, read_y).is_none() {
                    full = false;
                    break;
                }
            }
            if full {
                cleared += 1;
                continue; // skip copying this row (it's removed)
            }
            if write_y != read_y {
                // move row down
                for x in 0..self.w {
                    let v = self.get(x, read_y);
                    self.set(x, write_y, v);
                }
            }
            write_y -= 1;
        }

        // Fill remaining top rows with empty
        for y in 0..=write_y {
            for x in 0..self.w {
                self.set(x, y, None);
            }
        }

        cleared
    }
}
