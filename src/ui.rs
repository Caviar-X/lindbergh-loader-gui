use crate::config::{LindberghConfig, SdlKeymap};
use crate::games::{GameData, GameTitle};
use eframe::egui::{self, Align, Button, Color32, Label, Layout, Margin, Modal, RichText, Spacing};
use rfd::FileDialog;
use std::path::PathBuf;
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
pub struct SharedState {
    pub new_game_modify: i32,
    pub shared_text: Vec<String>,
}
impl SharedState {
    pub fn assign_default_lindbergh_config(&mut self, config: &LindberghConfig) {
        self.shared_text[0] = config.exe_path.to_string();
        self.shared_text[1] = config.window_size.0.to_string();
        self.shared_text[2] = config.window_size.1.to_string();
    }
}
impl Default for SharedState {
    fn default() -> Self {
        Self {
            new_game_modify: -1,
            shared_text: vec![String::new(); 100],
        }
    }
}
pub struct LoaderApp {
    app_state: AppState,
    modal: Option<ModalInfo>,
    //TODO: mark this private when release : Testing purpose only
    pub game_library: Vec<GameData>,
    //dirty ways to share state TwT
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
                        Into::<GameData>::into(self.current_game.clone()).game_id
                    ));
                }
                ui.separator();
                egui::Grid::new("main page grid")
                    .num_columns(2)
                    .show(ui, |ui| {
                        let curr_data: GameData = self.current_game.clone().into();
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
                        if !curr_data.game_status && curr_data.game_title == String::from("Unkown")
                        {
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
                            {
                                if self.current_game != GameTitle::Unknown {
                                    self.app_state = AppState::ConfigureGame;
                                }
                            }
                            ui.end_row();
                            if ui
                                .button(RichText::new("Configure Mapping").size(15.0))
                                .clicked()
                            {
                                if self.current_game != GameTitle::Unknown {
                                    self.app_state = AppState::ConfigureMapping;
                                }
                            }
                            ui.end_row();
                            if ui
                                .button(RichText::new("Run the game").strong().size(15.0))
                                .clicked()
                            {
                                self.set_modal("cannot lah", ModalStatus::Error);
                            }
                            ui.end_row();
                            if ui
                                .button(RichText::new("Run Test").strong().size(15.0))
                                .clicked()
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
                            {
                                let mut del: i32 = -1;
                                for (cnt, i) in self.game_library.iter().enumerate() {
                                    if Into::<GameData>::into(self.current_game.clone()).game_title
                                        == i.game_title
                                    {
                                        del = cnt as i32;
                                    }
                                }
                                if del >= 0 {
                                    self.game_library.remove(del as usize);
                                }
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
                                            self.current_game = GameTitle::from(&i.game_title);
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
        if self.shared_state.new_game_modify == -1 {
            for (cnt, i) in self.game_library.iter().enumerate() {
                if i == &GameData::default() {
                    self.shared_state.new_game_modify = cnt as i32;
                }
            }
            if self.shared_state.new_game_modify == -1 {
                self.game_library.push(GameData::default());
                self.shared_state.new_game_modify = (self.game_library.len() - 1) as i32;
            }
        }
        let modf_pos: usize = self.shared_state.new_game_modify as usize;
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
                                .map(|x| GameTitle::from(&x.game_title))
                                .collect::<Vec<GameTitle>>()
                                .contains(&i)
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
            egui::TopBottomPanel::bottom("new game bottom panel")
                .show_separator_line(false)
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        if ui.button("Save").clicked()
                            && self.current_game != GameTitle::Unknown
                            && !self
                                .game_library
                                .iter()
                                .map(|x| GameTitle::from(&x.game_title))
                                .collect::<Vec<GameTitle>>()
                                .contains(&self.current_game)
                        {
                            self.game_library[modf_pos].assign_title(self.current_game.clone());
                            self.shared_state.new_game_modify = -1;
                            self.app_state = AppState::MainPage;
                        }
                    });
                });
        });
    }
    fn configure_game_ui(&mut self, ctx: &egui::Context) {
        for (cnt, i) in self.game_library.iter().enumerate() {
            if GameTitle::from(&i.game_title) == self.current_game {
                self.shared_state.new_game_modify = cnt as i32;
            }
        }
        if self.shared_state.new_game_modify == -1 {
            self.set_modal(
                "Oops!\nLooks like we don't know which game you're configuring.",
                ModalStatus::Error,
            );
            self.app_state = AppState::MainPage;
            return;
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            egui_alignments::top_horizontal(ui, |ui| {
                ui.heading(RichText::new("Configure Game").size(35.0).strong());
            });
            ui.separator();
            egui_alignments::top_horizontal_wrapped(ui, |ui| {
                egui::Grid::new("configure game grid").show(ui, |ui| {
                    ui.label("Executable Path:");
                    if ui.small_button("📁").clicked() {
                        if let Some(path) = FileDialog::new()
                            .add_filter("Executable File", &["elf", ""])
                            .pick_file()
                        {
                            self.shared_state.shared_text[0] = path.to_string_lossy().to_string();
                        }
                    }
                    ui.end_row();
                    ui.spacing_mut().item_spacing.x = Spacing::default().item_spacing.x;
                    ui.label("Window size preset:");
                    egui::ComboBox::from_id_salt("confgame combobox")
                        .width(50.0)
                        .selected_text(format!(
                            "{}x{}",
                            self.game_library[self.shared_state.new_game_modify as usize]
                                .config
                                .window_size
                                .0,
                            self.game_library[self.shared_state.new_game_modify as usize]
                                .config
                                .window_size
                                .1
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
                                    &mut self.game_library
                                        [self.shared_state.new_game_modify as usize]
                                        .config
                                        .window_size,
                                    i,
                                    format!("{}x{}", i.0, i.1),
                                );
                            }
                        });
                    ui.end_row();
                    ui.label("Or customize it.");
                    ui.end_row();
                    ui.label("The size will follow custom's by default.");
                    ui.end_row();
                    ui.label("Window width:");
                    ui.text_edit_singleline(&mut self.shared_state.shared_text[1]);

                    ui.end_row();
                    ui.label("Window Height:");
                    ui.text_edit_singleline(&mut self.shared_state.shared_text[2]);
                    ui.end_row();

                    ui.end_row();

                });
            });
        });
    }
}
impl eframe::App for LoaderApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.modal_update(ctx);
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
            AppState::ConfigureMapping => {}
        }
    }
}
