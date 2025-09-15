use std::time::Instant;
use egui::{Context, Frame, Stroke, Margin, RichText, Color32};

use crate::ui;
use crate::ui::theme::{apply as apply_theme, palette, ThemeKind};
use crate::game::{Game, GameInput, BOARD_W, BOARD_H};
use crate::audio::MusicManager;

pub struct App {
    theme: ThemeKind,
    game: Game,
    last: Instant,
    ghost_enabled: bool,
    gravity_pulse_mode: bool,
    music_manager: Option<MusicManager>,
    high_score: u64,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let theme = ThemeKind::Dark;
        apply_theme(theme, &cc.egui_ctx);
        
        // Initialize music manager
        let music_dirs = vec!["assets/Songs", "assets/songs"];
        let music_manager = match MusicManager::new(music_dirs, 4.0, 0.7, 0.15, 1.0) {
            Ok(mut manager) => {
                if let Err(e) = manager.start() {
                    eprintln!("Failed to start music: {}", e);
                }
                Some(manager)
            }
            Err(e) => {
                eprintln!("Failed to initialize music manager: {}", e);
                None
            }
        };
        
        Self { 
            theme, 
            game: Game::new(), 
            last: Instant::now(),
            ghost_enabled: true,
            gravity_pulse_mode: false,
            music_manager,
            high_score: 0,
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

        // Handle music controls
        ctx.input(|i| {
            if i.key_pressed(egui::Key::M) {
                if let Some(ref mut music_manager) = self.music_manager {
                    music_manager.toggle(!music_manager.is_enabled());
                }
            }
        });

        // Update music manager
        if let Some(ref mut music_manager) = self.music_manager {
            if let Err(e) = music_manager.update(dt * 1000.0) {
                eprintln!("Music update error: {}", e);
            }
        }
        
        // Update high score
        if self.game.score > self.high_score {
            self.high_score = self.game.score;
        }

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
                
                // Music section
                if let Some(ref mut music_manager) = self.music_manager {
                    stats_section(ui, &pal, "🎵 MUSIC", |ui| {
                        let current_track = music_manager.current_track();
                        if !current_track.is_empty() {
                            ui.label(
                                RichText::new(format!("♪ {}", current_track))
                                    .size(9.0)
                                    .color(pal.neon_magenta.gamma_multiply(0.8))
                            );
                        } else if music_manager.is_enabled() {
                            ui.label(
                                RichText::new("♪ Loading...")
                                    .size(9.0)
                                    .color(pal.text.gamma_multiply(0.6))
                            );
                        } else {
                            ui.label(
                                RichText::new("♪ Music Off")
                                    .size(9.0)
                                    .color(pal.text.gamma_multiply(0.6))
                            );
                        }
                        ui.label(
                            RichText::new(format!("Volume: {:.0}%", music_manager.get_master_volume() * 100.0))
                                .size(8.0)
                                .color(pal.text.gamma_multiply(0.7))
                        );
                        
                        // Music control buttons
                        ui.add_space(4.0);
                        ui.horizontal(|ui| {
                            // Mute/unmute button
                            let button_text = if music_manager.is_enabled() { "🔇 Mute" } else { "🔊 Unmute" };
                            if ui.button(RichText::new(button_text).size(9.0).color(pal.text)).clicked() {
                                music_manager.toggle(!music_manager.is_enabled());
                            }
                            
                            // Skip button
                            if ui.button(RichText::new("⏭ Skip").size(9.0).color(pal.text)).clicked() {
                                if let Err(e) = music_manager.skip_track() {
                                    eprintln!("Failed to skip track: {}", e);
                                }
                            }
                        });
                    });
                    ui.add_space(8.0);
                }
                
                // Controls help section
                stats_section(ui, &pal, "🎮 CONTROLS", |ui| {
                    ui.label(RichText::new("← → : Move").color(pal.text.gamma_multiply(0.8)).size(9.0));
                    ui.label(RichText::new("↓ : Soft Drop").color(pal.text.gamma_multiply(0.8)).size(9.0));
                    ui.label(RichText::new("Space : Hard Drop").color(pal.text.gamma_multiply(0.8)).size(9.0));
                    ui.label(RichText::new("Z/X : Rotate").color(pal.text.gamma_multiply(0.8)).size(9.0));
                    ui.label(RichText::new("P : Pause").color(pal.text.gamma_multiply(0.8)).size(9.0));
                    ui.label(RichText::new("R : Restart").color(pal.text.gamma_multiply(0.8)).size(9.0));
                    ui.label(RichText::new("M : Toggle Music").color(pal.text.gamma_multiply(0.8)).size(9.0));
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
                
                // Game Over overlay
                if self.game.game_over {
                    self.show_game_over_modal(ui, &pal);
                }
            });

        // Apply theme changes
        let should_be_dark = matches!(self.theme, ThemeKind::Dark);
        if ctx.style().visuals.dark_mode != should_be_dark {
            apply_theme(self.theme, ctx);
        }
    }
}

impl App {
    fn show_game_over_modal(&mut self, ui: &mut egui::Ui, pal: &crate::ui::theme::Palette) {
        // Create modal overlay
        let screen_rect = ui.available_rect_before_wrap();
        let modal_size = egui::vec2(400.0, 300.0);
        let modal_rect = egui::Rect::from_center_size(screen_rect.center(), modal_size);
        
        // Semi-transparent background
        ui.painter().rect_filled(
            screen_rect,
            egui::Rounding::ZERO,
            Color32::from_black_alpha(180)
        );
        
        // Modal window
        let mut child_ui = ui.new_child(egui::UiBuilder::new().max_rect(modal_rect));
        child_ui.scope(|ui| {
            ui.set_clip_rect(modal_rect);
            
            Frame::none()
                .fill(pal.bg1)
                .stroke(Stroke::new(3.0, pal.neon_magenta))
                .rounding(10.0)
                .inner_margin(Margin::same(20.0))
                .show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        // Game Over title
                        ui.label(
                            RichText::new("GAME OVER")
                                .size(28.0)
                                .color(pal.neon_magenta)
                                .strong()
                        );
                        
                        ui.add_space(15.0);
                        
                        // Final score
                        ui.label(
                            RichText::new(format!("Final Score: {:0>6}", self.game.score))
                                .size(18.0)
                                .color(pal.text)
                        );
                        
                        // Level achieved
                        ui.label(
                            RichText::new(format!("Level: {}", self.game.level + 1))
                                .size(14.0)
                                .color(pal.neon_cyan)
                        );
                        
                        // Lines cleared
                        ui.label(
                            RichText::new(format!("Lines: {}", self.game.lines))
                                .size(14.0)
                                .color(pal.text.gamma_multiply(0.9))
                        );
                        
                        ui.add_space(10.0);
                        
                        // High score notification
                        if self.game.score == self.high_score {
                            ui.label(
                                RichText::new("🎉 NEW HIGH SCORE! 🎉")
                                    .size(16.0)
                                    .color(pal.neon_magenta)
                                    .strong()
                            );
                            ui.add_space(10.0);
                        }
                        
                        ui.separator();
                        ui.add_space(10.0);
                        
                        ui.add_space(15.0);
                        
                        // Restart button
                        if ui.button(
                            RichText::new("Play Again")
                                .size(16.0)
                                .color(pal.text)
                        ).clicked() {
                            self.game = Game::new();
                            self.game.set_pulse_gravity(self.gravity_pulse_mode);
                        }
                    });
                });
        });
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
