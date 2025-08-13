use std::time::Instant;
use egui::{Context, Frame, Stroke, Margin, RichText, Color32};

use crate::ui;
use crate::ui::theme::{apply as apply_theme, palette, ThemeKind};
use crate::game::{Game, GameInput, BOARD_W, BOARD_H};

pub struct App {
    theme: ThemeKind,
    game: Game,
    last: Instant,
    ghost_enabled: bool,
    gravity_pulse_mode: bool,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let theme = ThemeKind::Dark;
        apply_theme(theme, &cc.egui_ctx);
        Self { 
            theme, 
            game: Game::new(), 
            last: Instant::now(),
            ghost_enabled: true,
            gravity_pulse_mode: false,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Timing and input
        let now = Instant::now();
        let dt = (now - self.last).as_secs_f32();
        self.last = now;

        let input = ctx.input(|i| GameInput {
            left: i.key_pressed(egui::Key::ArrowLeft),     // Changed to key_pressed for discrete movement
            right: i.key_pressed(egui::Key::ArrowRight),   // Changed to key_pressed for discrete movement
            soft_drop: i.key_down(egui::Key::ArrowDown),   // Keep key_down for continuous soft drop
            hard_drop: i.key_pressed(egui::Key::Space),
            rot_cw: i.key_pressed(egui::Key::X) || i.key_pressed(egui::Key::ArrowUp),
            rot_ccw: i.key_pressed(egui::Key::Z),
            pause: i.key_pressed(egui::Key::P),
            restart: i.key_pressed(egui::Key::R),
        });

        self.game.update(dt, input);
        ctx.request_repaint();
        
        let pal = palette(self.theme);
        
        // Enhanced right sidebar with synthwave styling
        egui::SidePanel::right("right_sidebar")
            .resizable(false)
            .exact_width(280.0)
            .frame(Frame::none()
                .fill(pal.bg1)
                .stroke(Stroke::new(2.0, pal.neon_cyan.gamma_multiply(0.3)))
                .inner_margin(Margin::same(16.0))
            )
            .show(ctx, |ui| {
                let old_gravity_pulse_mode = self.gravity_pulse_mode;
                let actions = ui::panel::right_panel(ui, &mut self.theme, self.game.paused, &mut self.ghost_enabled, &mut self.gravity_pulse_mode);
                
                // Update gravity pulse mode if it changed
                if old_gravity_pulse_mode != self.gravity_pulse_mode {
                    self.game.set_pulse_gravity(self.gravity_pulse_mode);
                }
                
                if actions.pause_toggled {
                    self.game.paused = !self.game.paused;
                }
                if actions.restart {
                    self.game = Game::new();
                    self.game.set_pulse_gravity(self.gravity_pulse_mode); // Apply current settings to new game
                }

                ui.add_space(10.0);
                
                // Next pieces preview
                stats_section(ui, &pal, "⏭ NEXT", |ui| {
                    let preview_pieces = self.game.preview_pieces(3);
                    for (i, piece) in preview_pieces.iter().enumerate() {
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new(format!("{}:", i + 1))
                                    .size(10.0)
                                    .color(pal.text.gamma_multiply(0.7))
                            );
                            ui::draw::preview_piece(ui, &pal, *piece, 12.0);
                        });
                        if i < 2 {
                            ui.add_space(2.0);
                        }
                    }
                });
                
                ui.add_space(8.0);
                
                // Enhanced game statistics section
                stats_section(ui, &pal, "📊 STATS", |ui| {
                    stat_row(ui, &pal, "Score", &format!("{:0>6}", self.game.score), pal.neon_magenta);
                    stat_row(ui, &pal, "Level", &format!("{}", self.game.level + 1), pal.neon_cyan);
                    stat_row(ui, &pal, "Lines", &format!("{}", self.game.lines), pal.text);
                    stat_row(ui, &pal, "Combo", "×0", Color32::from_rgb(255, 200, 100));
                });
                
                ui.add_space(8.0);
                
                // Controls help section
                stats_section(ui, &pal, "🎮 CONTROLS", |ui| {
                    ui.label(RichText::new("← → : Move").color(pal.text.gamma_multiply(0.8)).size(9.0));
                    ui.label(RichText::new("↓ : Soft Drop").color(pal.text.gamma_multiply(0.8)).size(9.0));
                    ui.label(RichText::new("Space : Hard Drop").color(pal.text.gamma_multiply(0.8)).size(9.0));
                    ui.label(RichText::new("Z/X : Rotate").color(pal.text.gamma_multiply(0.8)).size(9.0));
                    ui.label(RichText::new("P : Pause").color(pal.text.gamma_multiply(0.8)).size(9.0));
                    ui.label(RichText::new("R : Restart").color(pal.text.gamma_multiply(0.8)).size(9.0));
                });
            });

        // Enhanced main playfield with synthwave background
        egui::CentralPanel::default()
            .frame(Frame::none()
                .fill(pal.bg0)
                .inner_margin(Margin::same(20.0))
            )
            .show(ctx, |ui| {
                let ghost = if self.ghost_enabled {
                    Some(self.game.ghost_position())
                } else {
                    None
                };
                
                ui::draw::playfield(
                    ui, 
                    &pal, 
                    (BOARD_W as usize, BOARD_H as usize), 
                    self.game.board(), 
                    Some(self.game.active()),
                    ghost.as_ref(),
                    self.ghost_enabled
                );
            });

        // Apply theme changes
        let should_be_dark = matches!(self.theme, ThemeKind::Dark);
        if ctx.style().visuals.dark_mode != should_be_dark {
            apply_theme(self.theme, ctx);
        }
    }
}

fn stats_section<R>(
    ui: &mut egui::Ui,
    pal: &crate::ui::theme::Palette,
    title: &str,
    content: impl FnOnce(&mut egui::Ui) -> R,
) -> R {
    ui.label(
        RichText::new(title)
            .size(13.0)
            .color(pal.neon_cyan)
            .strong()
    );
    ui.separator();
    ui.add_space(3.0);
    
    let result = ui.indent("stats_content", |ui| {
        content(ui)
    }).inner;
    
    ui.add_space(4.0);
    result
}

fn stat_row(
    ui: &mut egui::Ui,
    pal: &crate::ui::theme::Palette,
    label: &str,
    value: &str,
    value_color: Color32,
) {
    ui.horizontal(|ui| {
        ui.label(RichText::new(format!("{}:", label)).color(pal.text.gamma_multiply(0.9)));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(RichText::new(value).color(value_color).strong());
        });
    });
}
