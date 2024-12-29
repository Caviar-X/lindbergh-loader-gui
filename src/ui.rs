use eframe::egui::{self, Color32, Modal, RichText};

enum AppState {
    MainPage,
    ConfigureMapping,
    ConfigureGame,
    NewGame,
}

enum ModalStatus {
    Error,
    Warning,
    Info,
}
struct ModalInfo {
    pub data: String,
    pub status: ModalStatus,
}

impl Default for AppState {
    fn default() -> Self {
        Self::MainPage
    }
}

#[derive(Default)]
pub struct LoaderApp {
    app_state: AppState,
    modal: Option<ModalInfo>,
}

impl LoaderApp {
    fn set_modal(&mut self, data: impl Into<String>, status: ModalStatus) {
        let i = ModalInfo {
            data: data.into(),
            status,
        };
        self.modal = Some(i);
    }
    fn modal_update(&mut self, ctx: &egui::Context) {
        if self.modal.is_some() {
            Modal::new(egui::Id::new("New Modal")).show(ctx, |ui| {
                let u = self.modal.as_ref().unwrap();
                ui.set_width(250.0);
                ui.horizontal(|ui| match u.status {
                    ModalStatus::Error => {
                        ui.colored_label(
                            Color32::from_rgb(255, 0, 0),
                            RichText::new("Error").strong().size(25.0),
                        );
                    }
                    ModalStatus::Info => {
                        ui.colored_label(
                            Color32::from_rgb(0, 0, 255),
                            RichText::new("Info").strong().size(25.0),
                        );
                    }
                    ModalStatus::Warning => {
                        ui.colored_label(
                            Color32::from_rgb(255, 255, 0),
                            RichText::new("Warning").strong().size(25.0),
                        );
                    }
                });
                ui.separator();
                ui.label(&u.data);
                if ui.button("close").clicked() {
                    self.modal = None;
                }
            });
        }
    }
}
impl LoaderApp {
    fn main_page_ui(&mut self, ctx: &egui::Context) {
        egui::SidePanel::right("main page right panel")
            .exact_width(250.0)
            .show(ctx, |ui| {
                ui.image(egui::include_image!("../assets/default.png"));
                ui.separator();
                egui::Grid::new("main page grid")
                    .num_columns(2)
                    .show(ui, |ui| {
                        ui.strong("Game:");
                        ui.label("Unselected");
                        ui.end_row();
                        ui.strong("Game ID:");
                        ui.label("Unseclected");
                        ui.end_row();
                        ui.strong("DVP");
                        ui.label("Unknown");
                        ui.end_row();
                        ui.strong("Status:");
                        ui.label("Unknown");
                        ui.end_row();
                        ui.strong("GPU Vendor:");
                        ui.label("Unknown");
                        ui.end_row();
                    });
                ui.separator();
                if ui
                    .button(RichText::new("Configure Game").size(15.0))
                    .clicked()
                {
                    self.app_state = AppState::ConfigureGame;
                }
                if ui
                    .button(RichText::new("Configure Mapping").size(15.0))
                    .clicked()
                {
                    self.app_state = AppState::ConfigureMapping;
                }
                if ui
                    .button(RichText::new("Run the game").strong().size(15.0))
                    .clicked()
                {
                    self.set_modal("cannot lah", ModalStatus::Error);
                }
            });
        egui::SidePanel::left("left panel")
            .exact_width(590.0)
            .show(ctx, |ui| {
                ui.label(RichText::new("Game Library").size(25.5).strong());
                ui.separator();
                egui::ScrollArea::vertical().show(ui, |ui| {
                    egui::Grid::new("left panel grid")
                        .striped(true)
                        .num_columns(2)
                        .spacing((ui.available_width() - 515.0, 3.0))
                        .show(ui, |ui| {
                            ui.selectable_label(
                                false,
                                RichText::monospace(
                                    "Virtual Fighter 5 Final Showdown REV B ver 6.0000".into(),
                                )
                                .size(15.5),
                            );
                            ui.button("❌");
                            ui.end_row();
                            ui.selectable_label(
                                false,
                                RichText::monospace("Game2".into()).size(16.5),
                            );
                            ui.button("❌");
                            ui.end_row();
                        });
                });
            });
    }
    fn new_game_ui(ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                egui::Grid::new("new game grid")
                    .num_columns(2)
                    .spacing([40.0, 10.0])
                    .show(ui, |ui| {
                        ui.label(RichText::new("Game Name:").strong().size(15.5));
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
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("NewGame");
                    if ui.button("back").clicked() {
                        self.app_state = AppState::MainPage;
                    }
                });
            }
            AppState::ConfigureGame => {}
            AppState::ConfigureMapping => {}
        }
    }
}
