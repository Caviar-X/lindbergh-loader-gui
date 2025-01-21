use crate::config::{
    self, GameRegion, GpuType, Keymap, LindberghColor, LindberghConfig, executable_path,
};
use crate::games::{self, GameData, GameTitle, GameType};
use anyhow::{Ok, anyhow};
use eframe::egui::{self, Color32, Key, Modal, RichText};
use rfd::FileDialog;
use std::fs::{self, remove_file};
use std::iter::repeat;

enum AppState {
    MainPage,
    ConfigureMapping,
    ConfigureGame,
    NewGame,
}
#[derive(PartialEq, Clone, Eq)]
enum ModalStatus {
    Error,
    Warning,
    Info,
    About,
}
#[derive(Clone)]
struct ModalInfo {
    pub data: String,
    pub status: ModalStatus,
}

impl Default for AppState {
    fn default() -> Self {
        Self::MainPage
    }
}
#[derive(Clone)]
pub struct SharedState {
    pub new_game_modify: Option<usize>,
    pub shared_text: Vec<String>,
}
impl Default for SharedState {
    fn default() -> Self {
        Self {
            new_game_modify: None,
            shared_text: vec![String::new(); 100],
        }
    }
}
impl SharedState {
    pub fn assign_conf(&self, lconf: &mut LindberghConfig) -> anyhow::Result<()> {
        if !self.shared_text[0].is_empty() && !self.shared_text[1].is_empty() {
            lconf.window_size = (
                self.shared_text[0].parse::<u32>()?,
                self.shared_text[1].parse::<u32>()?,
            );
        }
        if lconf.enable_fps_limiter {
            lconf.limit_fps_target = self.shared_text[2].parse()?;
        }
        if lconf.exe_path.is_empty() {
            Err(anyhow!("Unspecified executable path"))
        } else {
            Ok(())
        }
    }
}
pub struct LoaderApp {
    app_state: AppState,
    modal: Option<ModalInfo>,
    game_library: Vec<GameData>,
    // dirty ways to share state TwT
    shared_state: SharedState,
    current_game: GameTitle,
}
impl Default for LoaderApp {
    fn default() -> Self {
        Self {
            app_state: AppState::default(),
            modal: None,
            game_library: vec![],
            shared_state: SharedState::default(),
            current_game: GameTitle::Unknown,
        }
    }
}
impl LoaderApp {
    fn set_modal(&mut self, data: impl Into<String>, status: ModalStatus) {
        self.modal = Some(ModalInfo {
            data: data.into(),
            status,
        });
    }
    fn get_game(&self) -> &GameData {
        &self.game_library[self.shared_state.new_game_modify.unwrap()]
    }
    fn get_game_mut(&mut self) -> &mut GameData {
        &mut self.game_library[self.shared_state.new_game_modify.unwrap()]
    }
    fn get_config(&self) -> &LindberghConfig {
        &self.get_game().config
    }
    fn get_config_mut(&mut self) -> &mut LindberghConfig {
        &mut self.get_game_mut().config
    }
    fn modal_update(&mut self, ctx: &egui::Context) {
        if self.modal.is_some() {
            Modal::new(egui::Id::new("New Modal")).show(ctx, |ui| {
                ui.horizontal_top(|ui| {
                    ui.vertical_centered(|ui| match self.modal.clone().unwrap().status {
                        ModalStatus::Error => {
                            ui.colored_label(
                                Color32::from_rgb(255, 0, 0),
                                RichText::new("Error").strong().size(25.0),
                            );
                        },
                        ModalStatus::Info => {
                            ui.colored_label(
                                Color32::from_rgb(0, 0, 255),
                                RichText::new("Info").strong().size(25.0),
                            );
                        },
                        ModalStatus::Warning => {
                            ui.colored_label(
                                Color32::from_rgb(255, 255, 0),
                                RichText::new("Warning").strong().size(25.0),
                            );
                        },
                        ModalStatus::About => {
                            ui.label(RichText::new("About").strong().size(25.0));
                        }
                    });
                });
                ui.separator();
                let u = self.modal.clone().unwrap();
                if u.status != ModalStatus::About {
                    ui.label(&u.data);
                } else {
                    ui.horizontal_wrapped(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;
                        ui.label("This is a GUI for ");
                        ui.hyperlink_to("lindbergh-loader", "https://github.com/lindbergh-loader/lindbergh-loader");
                        ui.label(" developed by ");
                        ui.hyperlink_to("Synth Magic", "https://github.com/Caviar-X");
                        ui.label(" with ♥");
                    });
                    ui.heading("Special Thanks");
                    ui.label("The lindbergh loader team: You guys made this project useful and possible!");
                    ui.horizontal_wrapped(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;
                        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                        ui.label(": Thanks for developing this gui framework :)");
                    });
                    ui.horizontal_wrapped(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;
                        ui.label("And everybody who supported and advised me on ");
                        ui.hyperlink_to("Arcade Community", "https://arcade.community");
                    });
                }
                ui.vertical_centered(|ui| {
                    if ui.button("close").clicked() {
                        self.modal = None;
                    }
                });
            });
        }
    }
    fn game_library_update(&mut self) {
        if self.game_library.is_empty() {
            let r = executable_path::get_list();
            if let Err(e) = r {
                self.set_modal(
                    format!(
                        "Error occurred while reading ./config/exe_paths.conf:\n{}",
                        e
                    ),
                    ModalStatus::Error,
                );
            } else if let Result::Ok(e) = r {
                self.game_library = e;
            }
        }
    }
}
impl LoaderApp {
    fn main_page_ui(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("main page top panel").show(ctx, |ui| {
            ui.menu_button("About", |ui| {
                ui.close_menu();
                self.set_modal("", ModalStatus::About);
            });
            ui.allocate_space(ui.available_size());
        });
        egui::SidePanel::right("main page right panel")
            .exact_width(275.0)
            .show(ctx, |ui| {
                if self.current_game == GameTitle::Unknown {
                    ui.image("file://./assets/default.png");
                } else {
                    ui.image(format!(
                        "file://./assets/{}.png",
                        self.current_game.as_gamedata().game_id
                    ));
                }
                ui.separator();
                egui::Grid::new("main page grid")
                    .num_columns(2)
                    .show(ui, |ui| {
                        let curr_data: GameData = self.current_game.as_gamedata();
                        ui.strong("Game:");
                        let s = &curr_data.game_title;
                        if s.len() >= 25 {
                            ui.label(format!("{}...", s.split_at(21).0));
                        } else {
                            ui.label(s);
                        }
                        ui.end_row();
                        ui.strong("Game ID:");
                        ui.label(&curr_data.game_id);
                        ui.end_row();
                        ui.strong("DVP");
                        ui.label(&curr_data.game_dvp);
                        ui.end_row();
                        ui.strong("Status:");
                        if !curr_data.game_status && curr_data.game_title == *"Unkown" {
                            ui.label("Unkown");
                        } else if !curr_data.game_status {
                            ui.colored_label(Color32::from_rgb(255, 0, 0), "Not Working");
                        } else {
                            ui.colored_label(Color32::from_rgb(0, 128, 0), "Working");
                        }
                        ui.end_row();
                        ui.strong("Support ATI Driver");
                        if curr_data.not_working_on_ati {
                            ui.colored_label(Color32::from_rgb(255, 0, 0), "No");
                        } else {
                            ui.colored_label(Color32::from_rgb(0, 128, 0), "Yes");
                        }
                        ui.end_row();
                    });
                ui.separator();
                egui::Grid::new("buttons").num_columns(1).show(ui, |ui| {
                    ui.with_layout(
                        egui::Layout::top_down_justified(egui::Align::Center),
                        |ui| {
                            if ui
                                .button(RichText::new("Configure Game").size(15.0))
                                .clicked()
                                && self.current_game != GameTitle::Unknown
                            {
                                self.app_state = AppState::ConfigureGame;
                            }
                            ui.end_row();
                            if ui
                                .button(RichText::new("Configure Mapping").size(15.0))
                                .clicked()
                                && self.current_game != GameTitle::Unknown
                            {
                                self.app_state = AppState::ConfigureMapping;
                            }
                            ui.end_row();
                            if ui
                                .button(RichText::new("Run the game").strong().size(15.0))
                                .clicked()
                                && self.current_game != GameTitle::Unknown
                            {
                                self.set_modal("cannot lah", ModalStatus::Error);
                            }
                            ui.end_row();
                            if ui
                                .button(RichText::new("Run Test").strong().size(15.0))
                                .clicked()
                                && self.current_game != GameTitle::Unknown
                            {
                                self.set_modal("Placeholder", ModalStatus::Info);
                            }
                            ui.end_row();
                            if ui
                                .button(
                                    RichText::new("Delete from library")
                                        .color(Color32::from_rgb(255, 0, 0))
                                        .strong()
                                        .size(15.0),
                                )
                                .clicked()
                                && self.current_game != GameTitle::Unknown
                            {
                                let mut del: i32 = -1;
                                for (cnt, i) in self.game_library.iter().enumerate() {
                                    if self.current_game.as_gamedata().game_title == i.game_title {
                                        del = cnt as i32;
                                    }
                                }
                                if del >= 0 {
                                    self.game_library.remove(del as usize);
                                }
                                if let Err(e) = executable_path::remove_exe_path(&self.current_game)
                                {
                                    self.set_modal(
                                        format!(
                                            "Unable to delete game {} because:\n{}",
                                            self.current_game, e
                                        ),
                                        ModalStatus::Error,
                                    );
                                } else if let Err(e) =
                                    remove_file(format!("./config/{:?}.conf", self.current_game))
                                {
                                    self.set_modal(
                                        format!(
                                            "Unable to delete game {} because:\n{}",
                                            self.current_game, e
                                        ),
                                        ModalStatus::Error,
                                    );
                                }

                                self.current_game = GameTitle::default();
                            }
                        },
                    );
                });
            });
        egui::SidePanel::left("left panel")
            .exact_width(565.0)
            .show(ctx, |ui| {
                ui.horizontal_top(|ui| {
                    egui::Grid::new("top grid")
                        .num_columns(2)
                        .spacing([ui.available_width() - 255.0, 0.0])
                        .show(ui, |ui| {
                            ui.label(RichText::new("Game Library").size(25.5).strong());
                            if ui.button("➕📚Add Games").clicked() {
                                self.app_state = AppState::NewGame;
                            }
                            ui.end_row();
                        });
                });
                ui.separator();
                egui::ScrollArea::vertical()
                    .auto_shrink(false)
                    .show(ui, |ui| {
                        ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                            egui::Grid::new("left panel grid")
                                .striped(true)
                                .num_columns(1)
                                .show(ui, |ui| {
                                    for i in self.game_library.iter() {
                                        if ui
                                            .selectable_label(
                                                false,
                                                RichText::new(&i.game_title).size(15.5).monospace(),
                                            )
                                            .clicked()
                                        {
                                            self.current_game = GameTitle::from(i);
                                        }
                                        ui.end_row();
                                    }
                                    ui.allocate_space(ui.available_size());
                                });
                        });
                    });
            });
    }
    fn new_game_ui(&mut self, ctx: &egui::Context) {
        if self.shared_state.new_game_modify.is_none() {
            for (cnt, i) in self.game_library.iter().enumerate() {
                if i == &GameData::default() {
                    self.shared_state.new_game_modify = Some(cnt);
                }
            }
            if self.shared_state.new_game_modify.is_none() {
                self.game_library.push(GameData::default());
                self.shared_state.new_game_modify = Some(self.game_library.len() - 1);
            }
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_top(|ui| {
                ui.vertical_centered(|ui| {
                    ui.heading(RichText::new("New Game").size(35.0).strong());
                })
            });
            egui_alignments::center_horizontal_wrapped(ui, |ui| {
                ui.label("Game title:");
                egui::ComboBox::from_id_salt("combo box")
                    .selected_text(self.current_game.to_string())
                    .show_ui(ui, |ui| {
                        for i in GameTitle::all_variants() {
                            if !self
                                .game_library
                                .iter()
                                .map(GameTitle::from)
                                .collect::<Vec<GameTitle>>()
                                .contains(i)
                            {
                                ui.selectable_value(
                                    &mut self.current_game,
                                    i.clone(),
                                    i.to_string(),
                                );
                            }
                        }
                    });
            });
            egui::TopBottomPanel::bottom("new game bottom panel").show(ctx, |ui| {
                egui_alignments::center_horizontal(ui, |ui| {
                    if ui.button("Save").clicked()
                        && self.current_game != GameTitle::Unknown
                        && !self
                            .game_library
                            .iter()
                            .map(GameTitle::from)
                            .collect::<Vec<GameTitle>>()
                            .contains(&self.current_game)
                    {
                        // make compiler happy
                        self.game_library[self.shared_state.new_game_modify.unwrap()]
                            .assign_title(&self.current_game);
                        self.shared_state.new_game_modify = None;
                        self.app_state = AppState::MainPage;
                    }
                    if ui.button("Cancel").clicked() {
                        self.game_library
                            .remove(self.shared_state.new_game_modify.unwrap());
                        self.current_game = GameTitle::Unknown;
                        self.shared_state.new_game_modify = None;
                        self.app_state = AppState::MainPage;
                    }
                });
            });
        });
    }
    fn configure_game_ui(&mut self, ctx: &egui::Context) {
        for (cnt, i) in self.game_library.iter().enumerate() {
            if GameTitle::from(i) == self.current_game {
                self.shared_state.new_game_modify = Some(cnt);
            }
        }
        if self.shared_state.new_game_modify.is_none() {
            self.set_modal(
                "Oops!\nLooks like we don't know which game you're configuring.",
                ModalStatus::Error,
            );
            self.app_state = AppState::MainPage;
            return;
        }
        let cur_game = self.current_game.clone();
        let p = format!("./config/{:?}.conf", cur_game);
        if fs::exists(&p).unwrap() && self.get_config().exe_path.is_empty() {
            if let Err(e) = self
                .get_config_mut()
                .read_from_lindbergh_conf_by_title(&cur_game)
            {
                self.set_modal(format!("config {} exists,but error occurred while reading data:\n{}\nCurrent game: {:?}",&p,e,cur_game), ModalStatus::Error);
            } else {
                self.shared_state.shared_text[0] = self.get_config().window_size.0.to_string();
                self.shared_state.shared_text[1] = self.get_config().window_size.1.to_string();
                self.shared_state.shared_text[2] = self.get_config().limit_fps_target.to_string();
            }
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            egui_alignments::top_horizontal(ui, |ui| {
                ui.heading(RichText::new("Configure Game").size(35.0).strong());
            });
            ui.separator();
            egui::ScrollArea::vertical()
                .id_salt("Configure Game ScrollArea")
                .show(ui, |ui| {
                    egui_alignments::top_horizontal_wrapped(ui, |ui| {
                        egui::Grid::new("configure game grid").show(ui, |ui| {
                            ui.label("Executable Path:");
                            if self.get_config().exe_path.len() > 40 {
                                ui.label(format!(
                                    "{}..",
                                    &(self.get_config().exe_path)
                                        .chars()
                                        .take(48)
                                        .collect::<String>()
                                ));
                            } else {
                                ui.label(&self.get_config().exe_path);
                            }
                            if ui.small_button("📁").clicked() {
                                if let Some(path) = FileDialog::new()
                                    .add_filter("Executable File", &["elf", ""])
                                    .pick_file()
                                {
                                    self.get_config_mut().exe_path =
                                        path.to_string_lossy().to_string();
                                }
                            }
                            ui.end_row();
                            ui.label("Window Size");
                            ui.label("(The size will follow the custom's by default)");
                            ui.end_row();
                            ui.label("Preset:");
                            egui::ComboBox::from_id_salt("window size combobox")
                                .width(50.0)
                                .selected_text(format!(
                                    "{}x{}",
                                    self.get_game().config.window_size.0,
                                    self.get_game().config.window_size.1
                                ))
                                .show_ui(ui, |ui| {
                                    for i in [
                                        (640, 480),
                                        (800, 600),
                                        (1024, 768),
                                        (1280, 1024),
                                        (800, 480),
                                        (1024, 600),
                                        (1280, 768),
                                        (1360, 768),
                                    ] {
                                        ui.selectable_value(
                                            &mut self.get_config_mut().window_size,
                                            i,
                                            format!("{}x{}", i.0, i.1),
                                        );
                                    }
                                });
                            ui.end_row();
                            ui.label("Custom width:");
                            ui.text_edit_singleline(&mut self.shared_state.shared_text[0]);
                            ui.end_row();
                            ui.label("Custom Height:");
                            ui.text_edit_singleline(&mut self.shared_state.shared_text[1]);
                            ui.end_row();
                            ui.label("Fullscreen:");
                            ui.checkbox(&mut self.get_config_mut().fullscreen, "");
                            ui.end_row();
                            ui.label("Disable SDL:");
                            ui.checkbox(&mut self.get_config_mut().disable_sdl, "");
                            ui.end_row();
                            ui.label("Reigon:");
                            egui::ComboBox::from_id_salt("reigon combobox")
                                .selected_text(self.get_config().game_region.to_string())
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(
                                        &mut self.get_config_mut().game_region,
                                        GameRegion::JP,
                                        "JP",
                                    );
                                    ui.selectable_value(
                                        &mut self.get_config_mut().game_region,
                                        GameRegion::US,
                                        "US",
                                    );
                                    ui.selectable_value(
                                        &mut self.get_config_mut().game_region,
                                        GameRegion::EX,
                                        "EX",
                                    );
                                });
                            ui.end_row();
                            ui.label("Freeplay:");
                            ui.checkbox(&mut self.get_config_mut().freeplay, "");
                            ui.end_row();
                            ui.label("Emulate JVS:");
                            ui.checkbox(&mut self.get_config_mut().emulate_jvs, "");
                            ui.end_row();
                            if !self.get_config().emulate_jvs {
                                ui.label("Enter serial port:");
                                ui.text_edit_singleline(&mut self.get_config_mut().jvs_path);
                                ui.end_row();
                            }
                            if GameTitle::from(self.get_game()) == GameTitle::Lets_Go_Jungle_Special
                                || GameTitle::from(self.get_game())
                                    == GameTitle::The_House_Of_The_Dead_4_Special
                            {
                                ui.label("Emulate Rideboard:");
                                ui.checkbox(&mut self.get_config_mut().emulate_rideboard, "");
                                if !self.get_config().emulate_rideboard {
                                    ui.end_row();
                                    ui.label("Enter serial port:");
                                    ui.text_edit_singleline(
                                        &mut self.get_config_mut().serial_port1,
                                    );
                                }
                                ui.end_row();
                            }
                            if self.get_game().game_type == Some(GameType::DRIVING) {
                                ui.label("Emulate driveboard:");
                                ui.checkbox(&mut self.get_config_mut().emulate_driveboard, "");
                                if !self.get_config().emulate_driveboard {
                                    ui.end_row();
                                    ui.label("Enter serial port:");
                                    ui.text_edit_singleline(
                                        &mut self.get_config_mut().serial_port1,
                                    );
                                }
                                ui.end_row();
                            }
                            if GameTitle::from(self.get_game()) == GameTitle::Outrun_2_SP_SDX {
                                ui.label("Emulate motionboard:");
                                ui.checkbox(&mut self.get_config_mut().emulate_motionboard, "");
                                if !self.get_config().emulate_motionboard {
                                    ui.end_row();
                                    ui.label("Enter serial port:");
                                    ui.text_edit_singleline(
                                        &mut self.get_config_mut().serial_port2,
                                    );
                                }
                                ui.end_row();
                            }
                            ui.label("SRAM path:");
                            ui.text_edit_singleline(&mut self.get_config_mut().sram_path);
                            ui.end_row();
                            ui.label("EEPROM path:");
                            ui.text_edit_singleline(&mut self.get_config_mut().eeprom_path);
                            ui.end_row();
                            ui.label("GPU Vendor:");
                            egui::ComboBox::from_id_salt("gpuv cbb")
                                .selected_text(self.get_config().gpu_vendor.to_string())
                                .show_ui(ui, |ui| {
                                    for i in [
                                        GpuType::AMD,
                                        GpuType::ATI,
                                        GpuType::Intel,
                                        GpuType::Nvidia,
                                        GpuType::AutoDetect,
                                        GpuType::Unknown,
                                    ] {
                                        ui.selectable_value(
                                            &mut self.get_config_mut().gpu_vendor,
                                            i.clone(),
                                            (i).to_string(),
                                        );
                                    }
                                });
                            ui.end_row();
                            ui.label("Show debug message:");
                            ui.checkbox(&mut self.get_config_mut().debug_message, "");
                            ui.end_row();
                            if GameTitle::from(self.get_game()) == GameTitle::Hummer
                                || GameTitle::from(self.get_game()) == GameTitle::Hummer_Extreme
                                || GameTitle::from(self.get_game()) == GameTitle::Hummer_Extreme_MDX
                            {
                                ui.label("Hummer Flicker Fix:");
                                ui.checkbox(&mut self.get_config_mut().hammer_flicker_fix, "");
                                ui.end_row();
                            }
                            ui.label("Keep aspect ratio:");
                            ui.checkbox(&mut self.get_config_mut().keep_aspect_ratio, "");
                            ui.end_row();
                            if GameTitle::from(self.get_game()) == GameTitle::Outrun_2_SP_SDX {
                                ui.label("Glare effect:");
                                ui.checkbox(
                                    &mut self.get_config_mut().outrun_lens_glare_enable,
                                    "",
                                );
                                ui.end_row();
                            }
                            ui.label("Enable border:");
                            ui.checkbox(&mut self.get_config_mut().border_enabled, "");
                            ui.end_row();
                            if self.get_config().border_enabled {
                                ui.label("White border:");
                                ui.add(
                                    egui::Slider::new(
                                        &mut self.get_config_mut().white_border_percentage,
                                        0..=100,
                                    )
                                    .suffix("%"),
                                );
                                ui.end_row();
                                ui.label("Black border:");
                                ui.add(
                                    egui::Slider::new(
                                        &mut self.get_config_mut().black_border_percentage,
                                        0..=100,
                                    )
                                    .suffix("%"),
                                );
                                ui.end_row();
                            }
                            ui.label("Enable FPS limiter:");
                            ui.checkbox(&mut self.get_config_mut().enable_fps_limiter, "");
                            ui.end_row();
                            if self.get_config().enable_fps_limiter {
                                ui.label("FPS limit:");
                                ui.text_edit_singleline(&mut self.shared_state.shared_text[2]);
                                ui.end_row();
                            }
                            if GameTitle::from(self.get_game()) == GameTitle::Outrun_2_SP_SDX {
                                ui.label("Skip cabinet check:");
                                ui.checkbox(
                                    &mut self.get_config_mut().skip_outrun_cabinet_check,
                                    "",
                                );
                                ui.end_row();
                            }
                            if GameTitle::from(self.get_game()) == GameTitle::Taisen_Mahjong_4
                                || GameTitle::from(self.get_game())
                                    == GameTitle::Taisen_Mahjong_4_Evolution
                            {
                                ui.label("Mahjong 4 enable all the time:");
                                ui.checkbox(&mut self.get_config_mut().mj4_enable_all_time, "");
                                ui.end_row();
                            }
                            ui.label("Lindbergh color:");
                            egui::ComboBox::from_id_salt("color combobox")
                                .selected_text(self.get_config().lindbergh_color.to_string())
                                .show_ui(ui, |ui| {
                                    for i in [
                                        LindberghColor::BLUE,
                                        LindberghColor::RED,
                                        LindberghColor::REDEX,
                                        LindberghColor::SILVER,
                                        LindberghColor::YELLOW,
                                    ] {
                                        ui.selectable_value(
                                            &mut self.get_config_mut().lindbergh_color,
                                            i.clone(),
                                            i.to_string(),
                                        );
                                    }
                                });
                            ui.end_row();
                        });
                    });
                });
        });
        egui::TopBottomPanel::bottom("config game btm panel").show(ctx, |ui| {
            egui_alignments::center_horizontal(ui, |ui| {
                if ui.button("Save").clicked() {
                    let v = self.shared_state.clone();
                    if let Err(e) = v.assign_conf(self.get_config_mut()) {
                        self.set_modal(
                            format!("Error occurred while parsing data \"{}\"", e),
                            ModalStatus::Error,
                        );
                    } else if let Err(e) = self
                        .get_config()
                        .write_to_lindbergh_conf(&self.current_game)
                    {
                        self.set_modal(
                            format!("Error occurred while writing data \"{}\"", e),
                            ModalStatus::Error,
                        );
                    } else {
                        self.set_modal(
                            format!(
                                "Configuration successfully saved into ./config/{:?}.conf",
                                self.current_game
                            ),
                            ModalStatus::Info,
                        );
                        self.shared_state.new_game_modify = None;
                        self.shared_state.shared_text = vec![String::new(); 100];
                        self.app_state = AppState::MainPage;
                    }
                }
                if ui.button("Cancel").clicked() {
                    self.shared_state.new_game_modify = None;
                    self.shared_state.shared_text = vec![String::new(); 100];
                    self.app_state = AppState::MainPage;
                }
            });
        });
    }
    fn configure_mapping_ui(&mut self, ctx: &egui::Context) {
        for (cnt, i) in self.game_library.iter().enumerate() {
            if GameTitle::from(i) == self.current_game {
                self.shared_state.new_game_modify = Some(cnt);
            }
        }
        if self.shared_state.new_game_modify.is_none() {
            self.set_modal(
                "Oops!\nLooks like we don't know which game you're configuring.",
                ModalStatus::Error,
            );
            self.app_state = AppState::MainPage;
            return;
        }
        // TODO: KILL THIS WITH FIRE
        let ohno = self.game_library[self.shared_state.new_game_modify.unwrap()]
            .config
            .input_method
            .clone();
        egui::CentralPanel::default().show(ctx, |ui| {
            egui_alignments::top_horizontal(ui, |ui| {
                ui.heading(RichText::new("Configure KeyMap").size(35.0).strong());
            });
            ui.separator();
            egui_alignments::top_horizontal(ui, |ui| {
                egui::Grid::new("configure mapping grid").show(ui, |ui| {
                    ui.label("Input Method");
                    // I've committed a sin,please don't kill me ferris
                    egui::ComboBox::from_id_salt("configure mapping combobox")
                        .selected_text(format!("{:?}", ohno))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.get_config_mut().input_method,
                                ohno.clone().into_both(),
                                "Both",
                            );
                            ui.selectable_value(
                                &mut self.get_config_mut().input_method,
                                ohno.clone().into_sdl(),
                                "SDL/X11",
                            );
                            ui.selectable_value(
                                &mut self.get_config_mut().input_method,
                                ohno.clone().into_evdev(),
                                "Evdev",
                            );
                        });
                    let ohno = self.game_library[self.shared_state.new_game_modify.unwrap()]
                        .config
                        .input_method
                        .clone();
                    ui.end_row();
                    ui.end_row();
                    if self.get_config().input_method.has_sdl() {
                        let sdl_keymap = ohno.get_sdlkeymap().unwrap();
                        let mut_sdl_keymap = self
                            .get_config_mut()
                            .input_method
                            .get_sdlkeymap_mut()
                            .unwrap();
                        ui.strong("SDL/X11 Keymap:");
                        ui.end_row();
                        ui.label("Test Key:");
                        if ui.button(sdl_keymap.test.unwrap().name()).hovered() {
                            for k in Key::ALL {
                                if ctx.input(|i| i.key_down(*k)) {
                                    mut_sdl_keymap.test = Some(*k);
                                }
                            }
                        }
                        ui.end_row();
                        ui.label("Player 1 Start Key:");
                        if ui.button(sdl_keymap.start.name()).hovered() {
                            for k in Key::ALL {
                                if ctx.input(|i| i.key_down(*k)) {
                                    mut_sdl_keymap.start = *k;
                                }
                            }
                        }
                        ui.end_row();
                        ui.label("Player 1 Service Key:");
                        if ui.button(sdl_keymap.service.name()).hovered() {
                            for k in Key::ALL {
                                if ctx.input(|i| i.key_down(*k)) {
                                    mut_sdl_keymap.service = *k;
                                    break;
                                }
                            }
                        }
                        ui.end_row();
                        ui.label("Player 1 Coin Key:");
                        if ui.button(sdl_keymap.coin.unwrap().name()).hovered() {
                            for k in Key::ALL {
                                if ctx.input(|i| i.key_down(*k)) {
                                    mut_sdl_keymap.coin = Some(*k);
                                    break;
                                }
                            }
                        }
                        ui.end_row();
                        ui.label("Player 1 Up Key:");
                        if ui.button(sdl_keymap.up.name()).hovered() {
                            for k in Key::ALL {
                                if ctx.input(|i| i.key_down(*k)) {
                                    mut_sdl_keymap.up = *k;
                                    break;
                                }
                            }
                        }
                        ui.end_row();
                        ui.label("Player 1 Down Key:");
                        if ui.button(sdl_keymap.down.name()).hovered() {
                            for k in Key::ALL {
                                if ctx.input(|i| i.key_down(*k)) {
                                    mut_sdl_keymap.down = *k;
                                    break;
                                }
                            }
                        }
                        ui.end_row();
                        ui.label("Player 1 Left Key:");
                        if ui.button(sdl_keymap.left.name()).hovered() {
                            for k in Key::ALL {
                                if ctx.input(|i| i.key_down(*k)) {
                                    mut_sdl_keymap.left = *k;
                                    break;
                                }
                            }
                        }
                        ui.end_row();
                        ui.label("Player 1 Right Key:");
                        if ui.button(sdl_keymap.right.name()).hovered() {
                            for k in Key::ALL {
                                if ctx.input(|i| i.key_down(*k)) {
                                    mut_sdl_keymap.right = *k;
                                    break;
                                }
                            }
                        }
                        ui.end_row();
                        ui.label("Player 1 Button 1 Key:");
                        if ui.button(sdl_keymap.button1.name()).hovered() {
                            for k in Key::ALL {
                                if ctx.input(|i| i.key_down(*k)) {
                                    mut_sdl_keymap.button1 = *k;
                                    break;
                                }
                            }
                        }
                        ui.end_row();
                        ui.label("Player 1 Button 2 Key:");
                        if ui.button(sdl_keymap.button2.name()).hovered() {
                            for k in Key::ALL {
                                if ctx.input(|i| i.key_down(*k)) {
                                    mut_sdl_keymap.button2 = *k;
                                    break;
                                }
                            }
                        }
                        ui.end_row();
                        ui.label("Player 1 Button 3 Key:");
                        if ui.button(sdl_keymap.button3.name()).hovered() {
                            for k in Key::ALL {
                                if ctx.input(|i| i.key_down(*k)) {
                                    mut_sdl_keymap.button3 = *k;
                                    break;
                                }
                            }
                        }
                        ui.end_row();
                        ui.label("Player 1 Button 4 Key:");
                        if ui.button(sdl_keymap.button4.name()).hovered() {
                            for k in Key::ALL {
                                if ctx.input(|i| i.key_down(*k)) {
                                    mut_sdl_keymap.button4 = *k;
                                    break;
                                }
                            }
                        }
                    }
                    ui.end_row();
                    if self.get_config().input_method.has_evdev() {
                        let evdev_keymap = ohno.get_evdev().unwrap();
                        let mut_evdev_keymap = self
                            .get_config_mut()
                            .input_method
                            .get_evdev_mut()
                            .unwrap();
                        ui.strong("Evdev Keymap:");
                        ui.end_row();
                        ui.label("Test Key:");
                        ui.text_edit_singleline(mut_evdev_keymap.player1.test.as_mut().unwrap());
                        ui.end_row();
                        ui.label("Player 1 Start Key:");
                        ui.text_edit_singleline(&mut mut_evdev_keymap.player1.start);
                    }
                });
            });
        });
    }
}
impl eframe::App for LoaderApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.modal_update(ctx);
        self.game_library_update();
        match self.app_state {
            AppState::MainPage => {
                self.main_page_ui(ctx);
            }
            AppState::NewGame => {
                self.new_game_ui(ctx);
            }
            AppState::ConfigureGame => {
                self.configure_game_ui(ctx);
            }
            AppState::ConfigureMapping => {
                self.configure_mapping_ui(ctx);
            }
        }
    }
}

// NOTE: F13~F35 will not be mapped because I don't know in which universe keyboard has these keys

pub fn egui_key_to_keycode(key: &egui::Key) -> Option<u32> {
    let zero = key.name().chars().next().unwrap();
    if zero.is_numeric() {
        if zero == '0' {
            return Some(19);
        } else {
            return Some(((zero as u8) - 38) as u32);
        }
    }
    if zero.is_alphabetic() && key.name().len() == 1 {
        return match zero {
            'Q' => Some(24),
            'W' => Some(25),
            'E' => Some(26),
            'R' => Some(27),
            'T' => Some(28),
            'Y' => Some(29),
            'U' => Some(30),
            'I' => Some(31),
            'O' => Some(32),
            'P' => Some(33),
            'A' => Some(38),
            'S' => Some(39),
            'D' => Some(40),
            'F' => Some(41),
            'G' => Some(42),
            'H' => Some(43),
            'J' => Some(44),
            'K' => Some(45),
            'L' => Some(46),
            'Z' => Some(52),
            'X' => Some(53),
            'C' => Some(54),
            'V' => Some(55),
            'B' => Some(56),
            'N' => Some(57),
            'M' => Some(58),
            _ => None,
        };
    }
    if zero == 'F' && key.name().len() > 1 {
        return match key {
            Key::F1 => Some(67),
            Key::F2 => Some(68),
            Key::F3 => Some(69),
            Key::F4 => Some(70),
            Key::F5 => Some(71),
            Key::F6 => Some(72),
            Key::F7 => Some(73),
            Key::F8 => Some(74),
            Key::F9 => Some(75),
            Key::F10 => Some(76),
            Key::F11 => Some(95),
            Key::F12 => Some(96),
            _ => None,
        };
    }
    match key {
        Key::Minus => Some(20),
        Key::Equals => Some(21),
        Key::Backspace => Some(22),
        Key::Tab => Some(23),
        Key::Semicolon => Some(47),
        Key::Quote => Some(48),
        Key::Backtick => Some(49),
        Key::Backslash => Some(51),
        Key::Comma => Some(59),
        Key::Period => Some(60),
        Key::Slash => Some(61),
        Key::Space => Some(65),
        Key::Home => Some(110),
        Key::ArrowUp => Some(111),
        Key::PageUp => Some(112),
        Key::ArrowLeft => Some(113),
        Key::ArrowRight => Some(114),
        Key::End => Some(115),
        Key::ArrowDown => Some(116),
        Key::PageDown => Some(117),
        Key::Insert => Some(118),
        Key::Delete => Some(119),
        Key::Escape => Some(9),
        Key::Enter => Some(36),
        Key::Colon => Some(47),
        Key::Plus => Some(21),
        Key::OpenBracket => Some(34),
        Key::CloseBracket => Some(35),
        Key::Pipe => Some(51),
        Key::Questionmark => Some(61),
        Key::Copy => Some(141),
        Key::Paste => Some(144),
        Key::Cut => Some(145),
        _ => None,
    }
}
// Thanks chatgpt
pub fn egui_keycode_to_key(keycode: u32) -> Option<egui::Key> {
    match keycode {
        // Numeric keys
        19 => Some(Key::Num0),
        10 => Some(Key::Num1),
        11 => Some(Key::Num2),
        12 => Some(Key::Num3),
        13 => Some(Key::Num4),
        14 => Some(Key::Num5),
        15 => Some(Key::Num6),
        16 => Some(Key::Num7),
        17 => Some(Key::Num8),
        18 => Some(Key::Num9),
        20 => Some(Key::Minus),
        21 => Some(Key::Equals),
        // Alphabetic keys
        24 => Some(Key::Q),
        25 => Some(Key::W),
        26 => Some(Key::E),
        27 => Some(Key::R),
        28 => Some(Key::T),
        29 => Some(Key::Y),
        30 => Some(Key::U),
        31 => Some(Key::I),
        32 => Some(Key::O),
        33 => Some(Key::P),
        38 => Some(Key::A),
        39 => Some(Key::S),
        40 => Some(Key::D),
        41 => Some(Key::F),
        42 => Some(Key::G),
        43 => Some(Key::H),
        44 => Some(Key::J),
        45 => Some(Key::K),
        46 => Some(Key::L),
        52 => Some(Key::Z),
        53 => Some(Key::X),
        54 => Some(Key::C),
        55 => Some(Key::V),
        56 => Some(Key::B),
        57 => Some(Key::N),
        58 => Some(Key::M),
        // Function keys
        67 => Some(Key::F1),
        68 => Some(Key::F2),
        69 => Some(Key::F3),
        70 => Some(Key::F4),
        71 => Some(Key::F5),
        72 => Some(Key::F6),
        73 => Some(Key::F7),
        74 => Some(Key::F8),
        75 => Some(Key::F9),
        76 => Some(Key::F10),
        95 => Some(Key::F11),
        96 => Some(Key::F12),
        // Punctuation keys
        34 => Some(Key::OpenBracket),
        35 => Some(Key::CloseBracket),
        47 => Some(Key::Semicolon),
        48 => Some(Key::Quote),
        49 => Some(Key::Backtick),
        51 => Some(Key::Backslash),
        59 => Some(Key::Comma),
        60 => Some(Key::Period),
        61 => Some(Key::Slash),
        65 => Some(Key::Space),
        // Navigation keys
        9 => Some(Key::Escape),
        22 => Some(Key::Backspace),
        23 => Some(Key::Tab),
        36 => Some(Key::Enter),
        110 => Some(Key::Home),
        111 => Some(Key::ArrowUp),
        112 => Some(Key::PageUp),
        113 => Some(Key::ArrowLeft),
        114 => Some(Key::ArrowRight),
        115 => Some(Key::End),
        116 => Some(Key::ArrowDown),
        117 => Some(Key::PageDown),
        118 => Some(Key::Insert),
        119 => Some(Key::Delete),
        // Clipboard actions
        141 => Some(Key::Copy),
        144 => Some(Key::Paste),
        145 => Some(Key::Cut),
        // Default case
        _ => None,
    }
}
