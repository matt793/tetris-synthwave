mod ui;
mod game;
mod app;

use egui::IconData;

fn main() -> eframe::Result<()> {
    // Load the application icon
    let icon_data = include_bytes!("../assets/Icon/Icon.ico");
    let icon = match image::load_from_memory(icon_data) {
        Ok(img) => {
            let img = img.to_rgba8();
            let (width, height) = img.dimensions();
            IconData {
                rgba: img.into_raw(),
                width: width as u32,
                height: height as u32,
            }
        }
        Err(err) => {
            eprintln!("Failed to load icon: {}", err);
            // Use default empty icon
            IconData {
                rgba: vec![],
                width: 0,
                height: 0,
            }
        }
    };

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 750.0])
            .with_min_inner_size([800.0, 650.0])
            .with_title("Tetris Synthwave")
            .with_icon(icon),
        ..Default::default()
    };

    eframe::run_native(
        "Tetris Synthwave",
        native_options,
        Box::new(|cc| Ok(Box::new(app::App::new(cc)))),
    )
}
