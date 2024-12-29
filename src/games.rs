use std::{fmt::Debug, process::Command};

pub enum GpuType {
    Nvidia,
    AMD,
    ATI,
    Intel,
    Unkown,
}
pub enum GameType {
    SHOOTING,
    DRIVING,
    HARLEY,
    FIGHTING,
    VT3,
    ABC,
}
pub enum LindberghColor {
    YELLOW,
    RED,
}

pub struct GameData {
    game_type: Option<GameType>,
    not_working_on_ati: bool,
    game_title: String,
    game_id: String,
    game_dvp: String,
    // true = Working false = Not working/Unkown
    game_status: bool,
    lindbergh_color: LindberghColor,
    executable_path: String,
}

impl Default for GameData {
    fn default() -> Self {
        Self {
            game_type: None,
            not_working_on_ati: false,
            game_title: "Unkown".into(),
            game_id: "Unkown".into(),
            game_dvp: "DVP-XXXX".into(),
            game_status: false,
            lindbergh_color: LindberghColor::YELLOW,
            executable_path: "".into(),
        }
    }
}
// Those variable names will be used to generate pretty names
// We do not care about revs
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq)]
enum GameTitle {
    After_Burner_Climax,
    // After_Burner_Climax_Rev_A,
    // After_Burner_Climax_Rev_B,
    After_Burner_Climax_SDX,
    // After_Burner_Climax_Sdx_Rev_A,
    After_Burner_Climax_SE,
    // After_Burner_Climax_Se_Rev_A,
    Ghost_Squad_Evolution,
    Harley_Davidson,
    Hummer,
    Hummer_SDLX,
    Hummer_Extreme,
    Hummer_Extreme_MDX,
    InitialD_4,
    // InitialD_4_Rev_A,
    // InitialD_4_Rev_B,
    // InitialD_4_Rev_C,
    // InitialD_4_Rev_D,
    // InitialD_4_Rev_G,
    InitalD_4_Export,
    // InitalD_4_Export_Rev_B,
    // InitalD_4_Export_Rev_C,
    // InitalD_4_Export_Rev_D,
    InitialD_5_Japan,
    // InitialD_5_Jap_Rev_A,
    // InitialD_5_Jap_Rev_F,
    InitalD_5_Export_Ver_2,
    InitalD_5_Export_Ver_4,
    Lets_Go_Jungle,
    // Lets_Go_Jungle_Rev_A,
    Lets_Go_Jungle_Special,
    Outrun_2_SP_SDX,
    // Outrun_2_Sp_Sdx_Rev_A,
    // Outrun_2_Sp_Sdx_Rev_A_Test,
    // Outrun_2_Sp_Sdx_Rev_A_Test2,
    Outrun_2_SP_SDX_Test,
    Primeval_Hunt,
    Rambo,
    Rambo_China,
    R_Tuned,
    Segaboot,
    Segaboot_2_4,
    Segaboot_2_4_With_Symbols,
    Segaboot_2_6,
    Sega_Race_TV,
    The_House_Of_The_Dead_4,
    The_House_Of_The_Dead_4_Test,
    // The_House_Of_The_Dead_4_Rev_A,
    // The_House_Of_The_Dead_4_Rev_A_Test,
    // The_House_Of_The_Dead_4_Rev_B,
    // The_House_Of_The_Dead_4_Rev_B_Test,
    // The_House_Of_The_Dead_4_Rev_C,
    // The_House_Of_The_Dead_4_Rev_C_Test,
    The_House_Of_The_Dead_4_Special,
    The_House_Of_The_Dead_4_Special_Test,
    // The_House_Of_The_Dead_4_Special_Rev_B,
    // The_House_Of_The_Dead_4_Special_Rev_B_Test,
    The_House_Of_The_Dead_EX,
    The_House_Of_The_Dead_EX_Test,
    Too_Spicy,
    Too_Spicy_Test,
    Virtua_Fighter_5,
    // Virtua_Fighter_5_Rev_A,
    // Virtua_Fighter_5_Rev_B,
    // Virtua_Fighter_5_Rev_E,
    Virtua_Fighter_5_Export,
    Virtua_Fighter_5_Final_Showdown,
    // Virtua_Fighter_5_Final_Showdown_Rev_A,
    // Virtua_Fighter_5_Final_Showdown_Rev_B,
    // Virtua_Fighter_5_Final_Showdown_Rev_B_6000,
    Virtua_Fighter_5_R,
    // Virtua_Fighter_5_R_Rev_D,
    // Virtua_Fighter_5_R_Rev_G,
    Virtua_Tennis_3,
    Virtua_Tennis_3_Test,
    // Virtua_Tennis_3_Rev_A,
    // Virtua_Tennis_3_Rev_A_Test,
    // Virtua_Tennis_3_Rev_B,
    // Virtua_Tennis_3_Rev_B_Test,
    // Virtua_Tennis_3_Rev_C,
    // Virtua_Tennis_3_Rev_C_Test,
    Unknown,
}
impl From<String> for GameTitle {
    fn from(value: String) -> Self {
        match value.as_str() {
            "After Burner Climax" => GameTitle::After_Burner_Climax,
            "After Burner Climax SDX" => GameTitle::After_Burner_Climax_SDX,
            "After Burner Climax SE" => GameTitle::After_Burner_Climax_SE,
            "Ghost Squad Evolution" => GameTitle::Ghost_Squad_Evolution,
            "Harley Davidson" => GameTitle::Harley_Davidson,
            "Hummer" => GameTitle::Hummer,
            "Hummer SDLX" => GameTitle::Hummer_SDLX,
            "Hummer Extreme" => GameTitle::Hummer_Extreme,
            "Hummer Extreme MDX" => GameTitle::Hummer_Extreme_MDX,
            "InitialD 4" => GameTitle::InitialD_4,
            "InitialD 4 Export" => GameTitle::InitalD_4_Export,
            "InitialD 5 Japan" => GameTitle::InitialD_5_Japan,
            "InitalD 5 Export Ver 2," => GameTitle::InitalD_5_Export_Ver_2,
            "InitalD 5 Export Ver 4" => GameTitle::InitalD_5_Export_Ver_4,
            "Segaboot 2 4" => GameTitle::Segaboot_2_4,
            "Segaboot 2 4 With Symbols" => GameTitle::Segaboot_2_4_With_Symbols,
            "Segaboot 2 6" => GameTitle::Segaboot_2_6,
            "Lets Go Jungle" => GameTitle::Lets_Go_Jungle,
            "Lets Go Jungle Special" => GameTitle::Lets_Go_Jungle_Special,
            "Outrun 2 SP SDX" => GameTitle::Outrun_2_SP_SDX,
            "Outrun 2 SP SDX Test" => GameTitle::Outrun_2_SP_SDX_Test,
            "Primeval Hunt" => GameTitle::Primeval_Hunt,
            "Rambo" => GameTitle::Rambo,
            "Rambo China" => GameTitle::Rambo_China,
            "R Tuned" => GameTitle::R_Tuned,
            "Segaboot" => GameTitle::Segaboot,
            "Sega Race TV" => GameTitle::Sega_Race_TV,
            "The House Of The Dead 4" => GameTitle::The_House_Of_The_Dead_4,
            "The House Of The Dead 4 Test" => GameTitle::The_House_Of_The_Dead_4_Test,
            "Too Spicy" => GameTitle::Too_Spicy,
            "Unknown" => GameTitle::Unknown,
            "Virtua Fighter 5" => GameTitle::Virtua_Fighter_5,
            "Virtua Tennis 3" => GameTitle::Virtua_Tennis_3,
            "The House Of The Dead 4 Special" => GameTitle::The_House_Of_The_Dead_4_Special,
            "The House Of The Dead 4 Special Test" => {
                GameTitle::The_House_Of_The_Dead_4_Special_Test
            }
            "The House Of The Dead EX" => GameTitle::The_House_Of_The_Dead_EX,
            "The House Of The Dead EX Test" => GameTitle::The_House_Of_The_Dead_EX_Test,
            "Too Spicy Test" => GameTitle::Too_Spicy_Test,
            "Virtua Fighter 5 Export" => GameTitle::Virtua_Fighter_5_Export,
            "Virtua Fighter 5 Final Showdown" => GameTitle::Virtua_Fighter_5_Final_Showdown,
            "Virtua Fighter 5 R" => GameTitle::Virtua_Fighter_5_R,
            "Virtua Tennis 3 Test" => GameTitle::Virtua_Tennis_3_Test,
            _ => GameTitle::Unknown,
        }
    }
}
impl Into<GameData> for GameTitle {
    fn into(self) -> GameData {
        match self {
            Self::Segaboot_2_4 | Self::Segaboot_2_4_With_Symbols => GameData {
                game_title: self.to_string(),
                game_status: true,
                ..Default::default()
            },
            Self::The_House_Of_The_Dead_4 | Self::The_House_Of_The_Dead_4_Test => GameData {
                game_title: self.to_string(),
                game_id: "SBLC".into(),
                game_type: Some(GameType::SHOOTING),
                game_status: true,
                game_dvp: "DVP-003B".into(),
                ..Default::default()
            },
            Self::The_House_Of_The_Dead_4_Special | Self::The_House_Of_The_Dead_4_Special_Test => {
                GameData {
                    game_title: self.to_string(),
                    game_id: "SBLS".into(),
                    game_type: Some(GameType::SHOOTING),
                    game_dvp: "DVP-0010".into(),
                    game_status: true,
                    ..Default::default()
                }
            }
            Self::The_House_Of_The_Dead_EX | Self::The_House_Of_The_Dead_EX_Test => GameData {
                game_title: self.to_string(),
                game_id: "SBRC".into(),
                game_type: Some(GameType::SHOOTING),
                game_dvp: "DVP-0063".into(),
                game_status: true,
                ..Default::default()
            },
            Self::Outrun_2_SP_SDX | Self::Outrun_2_SP_SDX_Test => GameData {
                game_title: self.to_string(),
                game_dvp: "DVP-0015".into(),
                game_id: "SBMB".into(),
                game_type: Some(GameType::DRIVING),
                game_status: true,
                ..Default::default()
            },
            Self::Virtua_Fighter_5 => GameData {
                game_title: self.to_string(),
                game_dvp: "DVP-0008".into(),
                game_id: "SBLM".into(),
                game_type: Some(GameType::FIGHTING),
                ..Default::default()
            },
            Self::Virtua_Fighter_5_Export => GameData {
                game_title: self.to_string(),
                game_dvp: "DVP-0043".into(),
                game_id: "SBLM".into(),
                game_status: true,
                game_type: Some(GameType::FIGHTING),
                ..Default::default()
            },
            Self::Virtua_Fighter_5_R => GameData {
                game_title: self.to_string(),
                game_id: "SBQU".into(),
                game_type: Some(GameType::FIGHTING),
                game_status: true,
                ..Default::default()
            },
            Self::Virtua_Fighter_5_Final_Showdown => GameData {
                game_title: self.to_string(),
                game_dvp: "DVP-SBUV".into(),
                game_id: "SBUV".into(),
                game_type: Some(GameType::FIGHTING),
                game_status: true,
                ..Default::default()
            },
            Self::Lets_Go_Jungle => GameData {
                game_title: "Let's Go Jungle! Lost on the Island of Spice!".into(),
                game_dvp: "DVP-0011".into(),
                not_working_on_ati: true,
                game_id: "SBLU".into(),
                game_status: true,
                game_type: Some(GameType::SHOOTING),
                ..Default::default()
            },
            Self::Lets_Go_Jungle_Special => GameData {
                game_title: "Let's Go Jungle! Special!".into(),
                game_dvp: "DVP-0036".into(),
                not_working_on_ati: true,
                game_id: "SBNR".into(),
                game_status: true,
                ..Default::default()
            },
            Self::After_Burner_Climax => GameData {
                game_title: self.to_string(),
                game_dvp: "DVP-0009".into(),
                not_working_on_ati: true,
                game_id: "SBLR".into(),
                game_type: Some(GameType::ABC),
                game_status: true,
                ..Default::default()
            },
            Self::After_Burner_Climax_SDX => GameData {
                game_title: self.to_string(),
                game_dvp: "DVP-0018-SDX".into(),
                game_id: "SBMN".into(),
                game_type: Some(GameType::ABC),
                game_status: true,
                ..Default::default()
            },
            Self::After_Burner_Climax_SE => GameData {
                game_title: self.to_string(),
                game_dvp: "DVP-0031".into(),
                game_id: "SBLR".into(),
                game_type: Some(GameType::ABC),
                game_status: true,
                ..Default::default()
            },
            Self::InitialD_4 => GameData {
                game_title: "Initial D Arcade Stage 4".into(),
                game_id: "SBML".into(),
                game_dvp: "DVP-0019".into(),
                game_type: Some(GameType::DRIVING),
                game_status: true,
                ..Default::default()
            },
            Self::InitalD_4_Export => GameData {
                game_title: "Initial D Arcade Stage 4 Export".into(),
                game_status: true,
                game_id: "SBNK".into(),
                game_dvp: "DVP-0030".into(),
                game_type: Some(GameType::DRIVING),
                ..Default::default()
            },
            Self::Sega_Race_TV => GameData {
                game_title: self.to_string(),
                game_dvp: "DVP-0044".into(),
                game_id: "SBPF".into(),
                game_type: Some(GameType::DRIVING),
                game_status: true,
                ..Default::default()
            },
            Self::Rambo | Self::Rambo_China => GameData {
                game_title: self.to_string(),
                game_dvp: "DVP-0069".into(),
                game_id: "SBQL".into(),
                game_type: Some(GameType::SHOOTING),
                game_status: true,
                ..Default::default()
            },
            Self::R_Tuned => GameData {
                game_title: self.to_string(),
                game_dvp: "DVP-0044".into(),
                game_id: "SBPF".into(),
                game_type: Some(GameType::SHOOTING),
                game_status: true,
                ..Default::default()
            },
            Self::Too_Spicy | Self::Too_Spicy_Test => GameData {
                game_title: self.to_string(),
                game_dvp: "DVP-0027".into(),
                game_id: "SBMV".into(),
                game_status: true,
                game_type: Some(GameType::SHOOTING),
                ..Default::default()
            },
            Self::Virtua_Tennis_3 | Self::Virtua_Tennis_3_Test => GameData {
                game_title: self.to_string(),
                game_dvp: "DVP-0005".into(),
                game_id: "SBKX".into(),
                game_type: Some(GameType::VT3),
                game_status: true,
                ..Default::default()
            },
            Self::Primeval_Hunt => GameData {
                game_title: self.to_string(),
                game_dvp: "DVP-0048".into(),
                game_type: Some(GameType::SHOOTING),
                game_id: "SBPP".into(),
                game_status: true,
                ..Default::default()
            },
            Self::Ghost_Squad_Evolution => GameData {
                game_title: self.to_string(),
                game_status: true,
                game_id: "SBNJ".into(),
                game_dvp: "DVP-0029A".into(),
                game_type: Some(GameType::SHOOTING),
                ..Default::default()
            },
            Self::InitialD_5_Japan => GameData {
                game_title: "Initial D Arcade Stage 5".into(),
                game_dvp: "DVP-0070".into(),
                game_id: "SBQZ".into(),
                game_status: true,
                game_type: Some(GameType::DRIVING),
                ..Default::default()
            },
            Self::InitalD_5_Export_Ver_2 => GameData {
                game_title: self.to_string(),
                game_status: true,
                game_dvp: "DVP-0075".into(),
                not_working_on_ati: true,
                game_id: "SBTS".into(),
                game_type: Some(GameType::DRIVING),
                ..Default::default()
            },
            Self::InitalD_5_Export_Ver_4 => GameData {
                game_title: self.to_string(),
                game_status: true,
                game_dvp: "DVP-0084".into(),
                game_id: "SBQN".into(),
                game_type: Some(GameType::DRIVING),
                ..Default::default()
            },
            Self::Hummer => GameData {
                game_title: self.to_string(),
                game_id: "SBQN".into(),
                game_dvp: "DVP-0057B".into(),
                game_type: Some(GameType::DRIVING),
                game_status: true,
                ..Default::default()
            },
            Self::Hummer_SDLX => GameData {
                game_title: self.to_string(),
                game_id: "SBST".into(),
                game_dvp: "DVP-0057".into(),
                game_type: Some(GameType::DRIVING),
                game_status: true,
                ..Default::default()
            },
            Self::Hummer_Extreme => GameData {
                game_title: self.to_string(),
                game_id: "SBST".into(),
                game_dvp: "DVP-0079".into(),
                game_type: Some(GameType::DRIVING),
                game_status: true,
                ..Default::default()
            },
            Self::Hummer_Extreme_MDX => GameData {
                game_title: self.to_string(),
                game_id: "SBST".into(),
                game_dvp: "DVP-0083".into(),
                game_type: Some(GameType::DRIVING),
                game_status: true,
                ..Default::default()
            },
            Self::Harley_Davidson => GameData {
                game_title: self.to_string(),
                game_type: Some(GameType::HARLEY),
                game_dvp: "DVP-5007".into(),
                game_id: "SBRG".into(),
                game_status: true,
                ..Default::default()
            },
            _ => GameData::default(),
        }
    }
}
impl ToString for GameTitle {
    fn to_string(&self) -> String {
        format!("{:?}", self).replace("_", " ")
    }
}
pub fn detect_gpu() -> Option<GpuType> {
    let op = Command::new("sh")
        .arg("-c")
        .arg("lspci | grep VGA | head -1")
        .output();
    if let Ok(res) = op {
        if !res.status.success() {
            return None;
        } else {
            let opstr = String::from_utf8(res.stdout);
            if let Ok(opstr) = opstr {
                let u = opstr.to_uppercase();
                if u.contains("NVIDIA") {
                    return Some(GpuType::Nvidia);
                } else if u.contains("INTEL") {
                    return Some(GpuType::Intel);
                } else if u.contains("AMD") && !u.contains("ATI") {
                    return Some(GpuType::AMD);
                } else if u.contains("ATI") && !u.contains("AMD") {
                    return Some(GpuType::ATI);
                } else if u.contains("AMD") && u.contains("ATI") {
                    let amdosd = Command::new("sh")
                        .arg("-c")
                        .arg("lsmod | grep amdgpu")
                        .output();
                    let amdld = Command::new("sh")
                        .arg("-c")
                        .arg("lsmod | grep radeon")
                        .output();
                    if let Ok(reso) = amdosd {
                        if !reso.status.success() {
                            return None;
                        }
                        if !reso.stdout.is_empty() {
                            return Some(GpuType::AMD);
                        } else {
                            if let Ok(resl) = amdld {
                                if !resl.status.success() {
                                    return None;
                                }
                                if !resl.stdout.is_empty() {
                                    return Some(GpuType::ATI);
                                } else {
                                    return Some(GpuType::Unkown);
                                }
                            } else {
                                return None;
                            }
                        }
                    } else {
                        return None;
                    }
                } else {
                    return Some(GpuType::Unkown);
                }
            } else {
                return None;
            }
        }
    } else {
        return None;
    }
}
