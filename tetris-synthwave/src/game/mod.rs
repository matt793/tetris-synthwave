pub mod piece;
pub mod random;
pub mod board;

use piece::{ActivePiece, Rot, Tetromino};
use random::SevenBag;
use board::Board;

/// Board dimensions (playable rows only; no hidden spawn rows yet)
pub const BOARD_W: i16 = 10;
pub const BOARD_H: i16 = 20;

/// Simple input snapshot from the UI layer.
/// We keep this minimal for the first playable shell.
#[derive(Default, Debug, Clone, Copy)]
pub struct GameInput {
    pub left: bool,
    pub right: bool,
    pub soft_drop: bool,
    pub hard_drop: bool,
    pub rot_cw: bool,
    pub rot_ccw: bool,
    pub pause: bool,
    pub restart: bool,
}

pub struct Game {
    pub paused: bool,
    pub score: u64,
    pub level: u32,
    pub lines: u32,

    active: ActivePiece,
    bag: SevenBag,
    board: Board,
    gravity_interval: f32, // seconds per row
    base_gravity_interval: f32, // base gravity before pulse effects
    acc: f32,
    pulse_time: f32, // time tracker for gravity pulse
    pulse_enabled: bool,
}

impl Game {
    pub fn new() -> Self {
        let mut bag = SevenBag::new(None);
        let board = Board::new();
        let active = Self::spawn(&mut bag, &board);
        let base_gravity = 0.8;
        Self {
            paused: false,
            score: 0,
            level: 0,
            lines: 0,
            active,
            bag,
            board,
            gravity_interval: base_gravity,
            base_gravity_interval: base_gravity,
            acc: 0.0,
            pulse_time: 0.0,
            pulse_enabled: false,
        }
    }

    pub fn set_pulse_gravity(&mut self, enabled: bool) {
        self.pulse_enabled = enabled;
        if !enabled {
            // Reset to base gravity when disabled
            self.gravity_interval = self.base_gravity_interval;
        }
    }

    fn spawn(bag: &mut SevenBag, board: &Board) -> ActivePiece {
        let t = bag.next();
        // Try to spawn centered. If initial pose is out of bounds due to offsets,
        // nudge downward a bit so it's visible.
        let mut p = ActivePiece::new(t, BOARD_W / 2, 0);
        // Adjust downwards up to 3 rows to get on-screen if some shapes have negative offsets.
        for _ in 0..3 {
            if !board.collides(&p) {
                break;
            }
            p.y += 1;
        }
        p
    }

    #[inline]
    fn can_place(&self, p: &ActivePiece) -> bool {
        !self.board.collides(p)
    }

    fn try_move(&mut self, dx: i16, dy: i16) -> bool {
        let mut np = self.active;
        np.x += dx;
        np.y += dy;
        if self.can_place(&np) {
            self.active = np;
            true
        } else {
            false
        }
    }

    fn try_rotate(&mut self, rot: Rot) -> bool {
        let mut np = self.active;
        np.rot = rot;
        if self.can_place(&np) {
            self.active = np;
            true
        } else {
            false
        }
    }

    fn lock_and_spawn(&mut self) {
        self.board.lock_piece(&self.active);
        let cleared = self.board.clear_full_lines();
        if cleared > 0 {
            self.lines += cleared;
            // very simple scoring for now
            self.score += match cleared {
                1 => 100,
                2 => 300,
                3 => 500,
                _ => 800,
            } as u64 * (self.level as u64 + 1);
            if self.lines / 10 > self.level {
                self.level += 1;
                // simple gravity ramp
                self.gravity_interval = (self.gravity_interval * 0.9).max(1.0 / 60.0);
            }
        }
        self.active = Self::spawn(&mut self.bag, &self.board);
        // If spawn immediately collides, treat as simple "top out": reset board and continue.
        if self.board.collides(&self.active) {
            self.board.clear();
            self.score = 0;
            self.level = 0;
            self.lines = 0;
        }
        self.acc = 0.0;
    }

    pub fn update(&mut self, dt: f32, input: GameInput) {
        if input.restart {
            *self = Self::new();
            return;
        }
        if input.pause {
            self.paused = !self.paused;
        }
        if self.paused {
            return;
        }

        // Rotate first (prioritize CCW over CW to avoid conflicts)
        if input.rot_ccw {
            let _ = self.try_rotate(self.active.rot.ccw());
        } else if input.rot_cw {
            let _ = self.try_rotate(self.active.rot.cw());
        }

        // Horizontal movement
        if input.left {
            let _ = self.try_move(-1, 0);
        }
        if input.right {
            let _ = self.try_move(1, 0);
        }

        // Hard drop: move down until collision, then lock
        if input.hard_drop {
            while self.try_move(0, 1) {}
            self.lock_and_spawn();
            return;
        }

        // Soft drop: try one row; if blocked, lock
        if input.soft_drop {
            if !self.try_move(0, 1) {
                self.lock_and_spawn();
                return;
            }
        }

        // Update pulse time for gravity oscillation
        self.pulse_time += dt;
        
        // Apply pulse gravity effect if enabled
        if self.pulse_enabled {
            // Sine wave oscillation with 8 second period (2π / 8 = π/4)
            let pulse_factor = (self.pulse_time * std::f32::consts::PI / 4.0).sin() * 0.5 + 1.0; // Oscillates between 0.5 and 1.5
            self.gravity_interval = self.base_gravity_interval * pulse_factor;
        }

        // Gravity
        self.acc += dt;
        if self.acc >= self.gravity_interval {
            self.acc -= self.gravity_interval;
            if !self.try_move(0, 1) {
                self.lock_and_spawn();
            }
        }
    }

    pub fn active(&self) -> &ActivePiece {
        &self.active
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    /// Calculate where the current piece would land (for ghost piece display)
    pub fn ghost_position(&self) -> ActivePiece {
        let mut ghost = self.active;
        // Drop the ghost piece until it would collide
        while !self.board.collides(&{
            let mut test_piece = ghost;
            test_piece.y += 1;
            test_piece
        }) {
            ghost.y += 1;
        }
        ghost
    }

    /// Get the next N pieces for preview
    pub fn preview_pieces(&mut self, count: usize) -> Vec<Tetromino> {
        self.bag.peek(count)
    }
}
