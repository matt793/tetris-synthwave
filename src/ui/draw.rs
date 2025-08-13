use egui::{pos2, Color32, Sense, Stroke, Ui, Vec2};

use super::theme::Palette;
use crate::game::{board::Board, piece::{ActivePiece, Tetromino}};

fn tet_color(t: Tetromino, pal: &Palette) -> Color32 {
    match t {
        Tetromino::I => Color32::from_rgb(0, 240, 240),
        Tetromino::O => Color32::from_rgb(240, 240, 0),
        Tetromino::T => pal.neon_magenta,
        Tetromino::S => Color32::from_rgb(0, 240, 0),
        Tetromino::Z => Color32::from_rgb(240, 0, 0),
        Tetromino::J => Color32::from_rgb(0, 0, 240),
        Tetromino::L => Color32::from_rgb(240, 160, 0),
    }
}

/// Draw a small preview tetromino piece
pub fn preview_piece(ui: &mut Ui, pal: &Palette, tetromino: crate::game::piece::Tetromino, size: f32) {
    let color = tet_color(tetromino, pal);
    let blocks = crate::game::piece::blocks(tetromino, crate::game::piece::Rot::R0);
    
    // Calculate bounds for centering
    let min_x = blocks.iter().map(|(x, _)| *x).min().unwrap_or(0);
    let max_x = blocks.iter().map(|(x, _)| *x).max().unwrap_or(0);
    let min_y = blocks.iter().map(|(_, y)| *y).min().unwrap_or(0);
    let max_y = blocks.iter().map(|(_, y)| *y).max().unwrap_or(0);
    
    let width = (max_x - min_x + 1) as f32 * size;
    let height = (max_y - min_y + 1) as f32 * size;
    
    let (resp, painter) = ui.allocate_painter(egui::Vec2::new(width, height), egui::Sense::hover());
    let rect = resp.rect;
    
    // Draw each block
    for (bx, by) in blocks {
        let x = rect.left() + ((bx - min_x) as f32) * size;
        let y = rect.top() + ((by - min_y) as f32) * size;
        let block_rect = egui::Rect::from_min_size(egui::pos2(x, y), egui::Vec2::splat(size));
        
        // Small glowing block
        painter.rect_filled(block_rect.shrink(1.0), 2.0, color);
        painter.rect_stroke(block_rect.shrink(1.0), 2.0, egui::Stroke::new(0.5, color.gamma_multiply(1.5)));
    }
}

/// Draw the playfield area with grid lines and game state
pub fn playfield(ui: &mut Ui, pal: &Palette, _dims: (usize, usize), board: &Board, active: Option<&ActivePiece>, ghost: Option<&ActivePiece>, show_ghost: bool) {
    let cols = board.w as usize;
    let rows = board.h as usize;

    // Calculate optimal cell size to use most of the available space
    let avail = ui.available_size();
    let margin = 30.0;
    
    let max_w = avail.x - margin;
    let max_h = avail.y - margin;
    
    let cell_w = max_w / cols as f32;
    let cell_h = max_h / rows as f32;
    let cell_size = cell_w.min(cell_h).max(25.0); // Larger minimum for better visibility
    
    let board_w = cell_size * cols as f32;
    let board_h = cell_size * rows as f32;
    
    // Center the board properly using egui's centering layout
    ui.vertical_centered(|ui| {
        ui.add_space((avail.y - board_h - margin) * 0.5);
        
        // Allocate the painter with the calculated size
        let (resp, painter) = ui.allocate_painter(Vec2::new(board_w, board_h), Sense::hover());
        let rect = resp.rect;
        
        draw_playfield_content(painter, rect, pal, board, active, ghost, show_ghost, cols, rows, cell_size);
    });
}

fn draw_playfield_content(
    painter: egui::Painter,
    rect: egui::Rect,
    pal: &Palette,
    board: &Board,
    active: Option<&ActivePiece>,
    ghost: Option<&ActivePiece>,
    show_ghost: bool,
    cols: usize,
    rows: usize,
    cell_size: f32,
) {

    // Synthwave background with gradient effect
    let bg_gradient = egui::epaint::RectShape::filled(
        rect,
        8.0,
        pal.bg0,
    );
    painter.add(bg_gradient);
    
    // Neon border with glow effect
    painter.rect_stroke(rect, 8.0, Stroke::new(3.0, pal.neon_magenta));
    painter.rect_stroke(rect, 8.0, Stroke::new(1.0, pal.neon_magenta.gamma_multiply(0.3)));

    // Subtle grid lines
    let grid_stroke = Stroke::new(0.8, pal.grid.gamma_multiply(0.6));
    
    // Vertical grid lines
    for x in 0..=cols {
        let x_pos = rect.left() + x as f32 * cell_size;
        painter.line_segment([pos2(x_pos, rect.top()), pos2(x_pos, rect.bottom())], grid_stroke);
    }
    
    // Horizontal grid lines
    for y in 0..=rows {
        let y_pos = rect.top() + y as f32 * cell_size;
        painter.line_segment([pos2(rect.left(), y_pos), pos2(rect.right(), y_pos)], grid_stroke);
    }
    
    // Prominent floor line with glow
    let floor_y = rect.bottom();
    painter.line_segment(
        [pos2(rect.left(), floor_y), pos2(rect.right(), floor_y)],
        Stroke::new(5.0, pal.neon_magenta),
    );
    painter.line_segment(
        [pos2(rect.left(), floor_y), pos2(rect.right(), floor_y)],
        Stroke::new(8.0, pal.neon_magenta.gamma_multiply(0.2)),
    );
 
    // Draw locked pieces with enhanced visuals
    for y in 0..rows {
        for x in 0..cols {
            if let Some(cell) = board.get(x as i16, y as i16) {
                let x0 = rect.left() + (x as f32) * cell_size;
                let y0 = rect.top() + (y as f32) * cell_size;
                let cell_rect = egui::Rect::from_min_size(pos2(x0, y0), Vec2::splat(cell_size));
                let color = tet_color(cell.t, pal);
                
                // Filled cell with rounded corners
                painter.rect_filled(cell_rect.shrink(2.0), 4.0, color);
                // Subtle inner glow
                painter.rect_stroke(cell_rect.shrink(2.0), 4.0, Stroke::new(1.0, color.gamma_multiply(1.5)));
                // Outer border
                painter.rect_stroke(cell_rect.shrink(1.0), 4.0, Stroke::new(0.5, pal.neon_cyan.gamma_multiply(0.8)));
            }
        }
    }

    // Draw ghost piece (where the piece will land) if enabled and not same as active position
    if show_ghost && ghost.is_some() && active.is_some() {
        let ghost_piece = ghost.unwrap();
        let active_piece = active.unwrap();
        
        // Only draw ghost if it's at a different position than active piece
        if ghost_piece.y != active_piece.y {
            let ghost_color = tet_color(ghost_piece.t, pal).gamma_multiply(0.4); // Semi-transparent
            for (dx, dy) in ghost_piece.cells() {
                if dx >= 0 && dy >= 0 && (dx as usize) < cols && (dy as usize) < rows {
                    let x0 = rect.left() + (dx as f32) * cell_size;
                    let y0 = rect.top() + (dy as f32) * cell_size;
                    let cell_rect = egui::Rect::from_min_size(pos2(x0, y0), Vec2::splat(cell_size));
                    
                    // Subtle ghost outline
                    painter.rect_stroke(cell_rect.shrink(3.0), 4.0, Stroke::new(1.5, ghost_color));
                }
            }
        }
    }
 
    // Draw active falling piece with glow effect
    if let Some(piece) = active {
        let color = tet_color(piece.t, pal);
        for (dx, dy) in piece.cells() {
            if dx >= 0 && dy >= 0 && (dx as usize) < cols && (dy as usize) < rows {
                let x0 = rect.left() + (dx as f32) * cell_size;
                let y0 = rect.top() + (dy as f32) * cell_size;
                let cell_rect = egui::Rect::from_min_size(pos2(x0, y0), Vec2::splat(cell_size));
                
                // Glowing active piece
                painter.rect_filled(cell_rect.shrink(2.0), 4.0, color);
                painter.rect_stroke(cell_rect.shrink(2.0), 4.0, Stroke::new(2.0, color.gamma_multiply(1.8)));
                painter.rect_stroke(cell_rect.shrink(1.0), 4.0, Stroke::new(1.0, pal.neon_cyan));
            }
        }
    }
}
