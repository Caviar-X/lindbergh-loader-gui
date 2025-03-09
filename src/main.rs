#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use eframe::egui;
use loader_gui::ui::LoaderApp;
use std::fs;

fn main() -> eframe::Result {
    let icon = eframe::icon_data::from_png_bytes(include_bytes!("../assets/default.png")).unwrap();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([840.0,1400.0])
            .with_maximize_button(false)
            .with_icon(icon)
            .with_decorations(true),
        ..Default::default()
    };
    // TODO: Find a way to report error
    if !fs::exists("./config").unwrap() {
        fs::create_dir("./config").unwrap();
    }
    if !fs::exists("./log").unwrap() {
        fs::create_dir("./log").unwrap();
    }
    if !fs::exists("./dynlibs").unwrap() {
        panic!("Unable to find lindbergh-loader's file");
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

