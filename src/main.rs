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
    let all_v: Vec<GameTitle> = vec![
        GameTitle::After_Burner_Climax,
        GameTitle::After_Burner_Climax_SDX,
        GameTitle::After_Burner_Climax_SE,
        GameTitle::Ghost_Squad_Evolution,
        GameTitle::Harley_Davidson,
        GameTitle::Hummer,
        GameTitle::Hummer_SDLX,
        GameTitle::Hummer_Extreme,
        GameTitle::Hummer_Extreme_MDX,
        GameTitle::InitialD_4,
        GameTitle::InitalD_4_Export,
        GameTitle::InitialD_5_Japan,
        GameTitle::InitalD_5_Export_Ver_2,
        GameTitle::InitalD_5_Export_Ver_4,
        GameTitle::Lets_Go_Jungle,
        GameTitle::Lets_Go_Jungle_Special,
        GameTitle::Outrun_2_SP_SDX,
        GameTitle::Primeval_Hunt,
        GameTitle::Rambo,
        GameTitle::Rambo_China,
        GameTitle::R_Tuned,
        GameTitle::Segaboot,
        GameTitle::Segaboot_2_4,
        GameTitle::Segaboot_2_4_With_Symbols,
        GameTitle::Segaboot_2_6,
        GameTitle::Sega_Race_TV,
        GameTitle::The_House_Of_The_Dead_4,
        GameTitle::The_House_Of_The_Dead_4_Special,
        GameTitle::The_House_Of_The_Dead_EX,
        GameTitle::Too_Spicy,
        GameTitle::Virtua_Fighter_5,
        GameTitle::Virtua_Fighter_5_Export,
        GameTitle::Virtua_Fighter_5_Final_Showdown,
        GameTitle::Virtua_Fighter_5_R,
        GameTitle::Virtua_Tennis_3,
    ];
    let mut tapp = LoaderApp::default();
    for i in all_v {
        tapp.game_library.push(i.into());
    }
    eframe::run_native(
        "Linderbergh loader GUI",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(tapp))
        }),
    )
}
