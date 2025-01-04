use eframe::egui;
use loader_gui::{games::GameTitle, ui::LoaderApp};
fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
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
    eframe::run_native(
        "Linderbergh loader GUI",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(LoaderApp::default()))
        }),
    )
}
