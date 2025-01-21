use eframe::egui;
use loader_gui::ui::LoaderApp;
use std::fs::{self, File};
fn main() -> eframe::Result {
    // Log to stderr (if you run with `RUST_LOG=debug`).
    let icon = eframe::icon_data::from_png_bytes(include_bytes!("../assets/default.png")).unwrap();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([840.0, 700.0])
            .with_resizable(false)
            .with_maximize_button(false)
            .with_icon(icon)
            .with_decorations(true),
        ..Default::default()
    };
    // TODO: Find a way to report error
    if !fs::exists("./config").unwrap() {
        fs::create_dir("./config").unwrap();
    }
    if !fs::exists("./config/exe_paths.conf").unwrap() {
        File::create("./config/exe_paths.conf").unwrap();
    }
    eframe::run_native(
        "Linderbergh loader GUI",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(LoaderApp::default()))
        }),
    )
}

#[cfg(not(target_os = "linux"))]
fn main() -> eframe::Result {
    compile_error!("This should not be compiled outside linux!")
}
