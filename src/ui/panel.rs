use egui::{Ui, RichText, Color32, Frame, Stroke, Margin};
use super::theme::{ThemeKind, palette};

pub struct PanelActions {
    pub pause_toggled: bool,
    pub restart: bool,
    pub ghost_toggled: bool,
    pub gravity_pulse_toggled: bool,
}

impl PanelActions {
    pub fn none() -> Self {
        Self {
            pause_toggled: false,
            restart: false,
            ghost_toggled: false,
            gravity_pulse_toggled: false,
        }
    }
}

pub fn right_panel(ui: &mut Ui, theme: &mut ThemeKind, paused: bool, ghost_enabled: &mut bool, gravity_pulse_mode: &mut bool) -> PanelActions {
    let mut actions = PanelActions::none();
    let pal = palette(*theme);

    // Enhanced synthwave styling for the entire panel
    let panel_frame = Frame::none()
        .fill(pal.bg0.gamma_multiply(0.8))
        .stroke(Stroke::new(1.0, pal.neon_cyan.gamma_multiply(0.5)))
        .rounding(8.0)
        .inner_margin(Margin::same(12.0));

    panel_frame.show(ui, |ui| {
        // Stylized heading
        ui.label(
            RichText::new("◆ TETRIS SYNTHWAVE ◆")
                .size(16.0)
                .color(pal.neon_magenta)
                .strong()
        );
        ui.add_space(8.0);

        // Controls section with neon styling
        control_section(ui, &pal, "⚡ CONTROLS", |ui| {
            ui.horizontal(|ui| {
                let pause_label = if paused { "▶ RESUME" } else { "⏸ PAUSE" };
                let pause_btn = ui.button(
                    RichText::new(pause_label)
                        .size(12.0)
                        .color(if paused { pal.neon_cyan } else { pal.neon_magenta })
                );
                if pause_btn.clicked() {
                    actions.pause_toggled = true;
                }

                let restart_btn = ui.button(
                    RichText::new("🔄 RESTART")
                        .size(12.0)
                        .color(Color32::from_rgb(255, 100, 100))
                );
                if restart_btn.clicked() {
                    actions.restart = true;
                }
            });
        });

        ui.add_space(12.0);

        // Toggles section
        control_section(ui, &pal, "⚙ TOGGLES", |ui| {
            if ui.checkbox(ghost_enabled, 
                RichText::new("👻 Ghost Piece")
                    .color(pal.text)
            ).clicked() {
                actions.ghost_toggled = true;
            }

            if ui.checkbox(gravity_pulse_mode, 
                RichText::new("🌊 Gravity: Pulse")
                    .color(pal.text)
            ).clicked() {
                actions.gravity_pulse_toggled = true;
            }
        });

        ui.add_space(12.0);

        // Theme section with enhanced styling
        control_section(ui, &pal, "🎨 THEME", |ui| {
            ui.horizontal(|ui| {
                let dark_selected = matches!(theme, ThemeKind::Dark);
                let light_selected = matches!(theme, ThemeKind::Light);

                if ui.selectable_label(dark_selected, 
                    RichText::new("🌙 Dark")
                        .color(if dark_selected { pal.neon_magenta } else { pal.text.gamma_multiply(0.7) })
                ).clicked() {
                    *theme = ThemeKind::Dark;
                }

                if ui.selectable_label(light_selected, 
                    RichText::new("☀ Light")
                        .color(if light_selected { pal.neon_magenta } else { pal.text.gamma_multiply(0.7) })
                ).clicked() {
                    *theme = ThemeKind::Light;
                }
            });
        });

        ui.add_space(12.0);

        // Preview section
        control_section(ui, &pal, "🔮 PREVIEW", |ui| {
            ui.label(
                RichText::new("Next: [3-piece queue]")
                    .color(pal.text.gamma_multiply(0.8))
                    .italics()
            );
        });
    });

    actions
}

fn control_section<R>(
    ui: &mut Ui,
    pal: &crate::ui::theme::Palette,
    title: &str,
    content: impl FnOnce(&mut Ui) -> R,
) -> R {
    // Section header with neon accent
    ui.label(
        RichText::new(title)
            .size(13.0)
            .color(pal.neon_cyan)
            .strong()
    );
    
    // Subtle divider line
    ui.separator();
    ui.add_space(4.0);
    
    // Content with slight indentation
    let result = ui.indent("section_content", |ui| {
        content(ui)
    }).inner;
    
    ui.add_space(6.0);
    result
}
