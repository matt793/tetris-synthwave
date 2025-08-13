use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tetromino {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rot {
    R0 = 0,
    R90 = 1,
    R180 = 2,
    R270 = 3,
}

impl Rot {
    pub fn cw(self) -> Self {
        match self {
            Rot::R0 => Rot::R90,
            Rot::R90 => Rot::R180,
            Rot::R180 => Rot::R270,
            Rot::R270 => Rot::R0,
        }
    }
    pub fn ccw(self) -> Self {
        match self {
            Rot::R0 => Rot::R270,
            Rot::R90 => Rot::R0,
            Rot::R180 => Rot::R90,
            Rot::R270 => Rot::R180,
        }
    }
}

// Precomputed block offsets for each tetromino at each rotation.
// Coordinates are in cells relative to the piece pivot position (x, y).
// These match common guideline shapes; SRS kicks will be added later.
// Order of minos is arbitrary but consistent per shape.
pub const SHAPES: [[[(i16, i16); 4]; 4]; 7] = {
    // I
    let i = [
        // R0
        [(-1, 0), (0, 0), (1, 0), (2, 0)],
        // R90
        [(1, -1), (1, 0), (1, 1), (1, 2)],
        // R180
        [(-1, 1), (0, 1), (1, 1), (2, 1)],
        // R270
        [(0, -1), (0, 0), (0, 1), (0, 2)],
    ];

    // O (pivot centered between 4 blocks; here we use offsets that render correctly)
    let o = [
        [(0, 0), (1, 0), (0, 1), (1, 1)],
        [(0, 0), (1, 0), (0, 1), (1, 1)],
        [(0, 0), (1, 0), (0, 1), (1, 1)],
        [(0, 0), (1, 0), (0, 1), (1, 1)],
    ];

    // T
    let t = [
        [(-1, 0), (0, 0), (1, 0), (0, 1)],
        [(0, -1), (0, 0), (0, 1), (1, 0)],
        [(-1, 0), (0, 0), (1, 0), (0, -1)],
        [(0, -1), (0, 0), (0, 1), (-1, 0)],
    ];

    // S
    let s = [
        [(-1, 0), (0, 0), (0, 1), (1, 1)],
        [(0, -1), (0, 0), (1, 0), (1, 1)],
        [(-1, -1), (0, -1), (0, 0), (1, 0)],
        [(-1, -1), (-1, 0), (0, 0), (0, 1)],
    ];

    // Z
    let z = [
        [(-1, 1), (0, 1), (0, 0), (1, 0)],
        [(1, -1), (1, 0), (0, 0), (0, 1)],
        [(-1, 0), (0, 0), (0, -1), (1, -1)],
        [(-1, -1), (-1, 0), (0, 0), (0, 1)],
    ];

    // J
    let j = [
        [(-1, 1), (-1, 0), (0, 0), (1, 0)],
        [(0, -1), (1, -1), (0, 0), (0, 1)],
        [(-1, 0), (0, 0), (1, 0), (1, -1)],
        [(0, -1), (0, 0), (-1, 1), (0, 1)],
    ];

    // L
    let l = [
        [(-1, 0), (0, 0), (1, 0), (1, 1)],
        [(0, -1), (0, 0), (0, 1), (1, 1)],
        [(-1, -1), (-1, 0), (0, 0), (1, 0)],
        [(-1, -1), (0, -1), (0, 0), (0, 1)],
    ];

    [i, o, t, s, z, j, l]
};

#[inline]
pub fn blocks(t: Tetromino, r: Rot) -> [(i16, i16); 4] {
    SHAPES[t as usize][r as usize]
}

#[derive(Debug, Clone, Copy)]
pub struct ActivePiece {
    pub t: Tetromino,
    pub rot: Rot,
    pub x: i16, // board coordinates
    pub y: i16,
}

impl ActivePiece {
    pub fn new(t: Tetromino, x: i16, y: i16) -> Self {
        Self { t, rot: Rot::R0, x, y }
    }
    pub fn cells(&self) -> [(i16, i16); 4] {
        let offs = blocks(self.t, self.rot);
        [
            (self.x + offs[0].0, self.y + offs[0].1),
            (self.x + offs[1].0, self.y + offs[1].1),
            (self.x + offs[2].0, self.y + offs[2].1),
            (self.x + offs[3].0, self.y + offs[3].1),
        ]
    }
}
