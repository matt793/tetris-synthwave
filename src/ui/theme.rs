use egui::{Color32, Context, Visuals};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ThemeKind {
    Dark,
    Light,
}

#[derive(Clone, Copy, Debug)]
pub struct Palette {
    pub bg0: Color32,
    pub bg1: Color32,
    pub text: Color32,
    pub grid: Color32,
    pub neon_cyan: Color32,
    pub neon_magenta: Color32,
}

pub fn palette(theme: ThemeKind) -> Palette {
    match theme {
        ThemeKind::Dark => Palette {
            // Deep navy/indigo
            bg0: Color32::from_rgb(12, 16, 32),
            // Slightly lighter for depth
            bg1: Color32::from_rgb(20, 28, 52),
            text: Color32::from_rgb(220, 225, 240),
            // Subtle cyan alpha for grid lines
            grid: Color32::from_rgba_premultiplied(0, 255, 255, 60),
            // Neon accents
            neon_cyan: Color32::from_rgb(0, 248, 255),
            neon_magenta: Color32::from_rgb(255, 0, 180),
        },
        ThemeKind::Light => Palette {
            // Softer gradient background - less bright
            bg0: Color32::from_rgb(235, 230, 245),
            bg1: Color32::from_rgb(220, 215, 235),
            text: Color32::from_rgb(40, 35, 60),
            // Much darker, more visible grid lines
            grid: Color32::from_rgba_premultiplied(80, 90, 120, 180),
            // More contrasted accents
            neon_cyan: Color32::from_rgb(20, 120, 160),
            neon_magenta: Color32::from_rgb(160, 20, 110),
        },
    }
}

// Apply egui visuals consistent with our palette.
// Keep it minimal; we will expand styling as UI grows.
pub fn apply(theme: ThemeKind, ctx: &Context) {
    let pal = palette(theme);
    let mut visuals = match theme {
        ThemeKind::Dark => Visuals::dark(),
        ThemeKind::Light => Visuals::light(),
    };

    // Background and text tuning
    visuals.override_text_color = Some(pal.text);
    visuals.panel_fill = pal.bg0;
    visuals.window_fill = pal.bg0;

    // Make widgets slightly rounded and crisp
    visuals.widgets.noninteractive.bg_fill = pal.bg1;
    visuals.widgets.inactive.fg_stroke.color = pal.text;

    ctx.set_visuals(visuals);
}
