use crate::{games::GameTitle, ui::{egui_key_to_keycode, egui_keycode_to_key}};
use anyhow::{Ok, anyhow};
use eframe::egui;
use std::{
    fmt::Display,
    fs::{self, File},
    io::Write,
    path::Path,
};
#[derive(Debug, PartialEq, Eq)]
pub enum GameRegion {
    JP,
    US,
    EX,
}
impl Display for GameRegion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum GpuType {
    AutoDetect = 0,
    Nvidia = 1,
    AMD = 2,
    ATI = 3,
    Intel = 4,
    Unknown = 5,
}
impl GpuType {
    fn into_i32(&self) -> i32 {
        match self {
            Self::AutoDetect => 0,
            Self::Nvidia => 1,
            Self::AMD => 2,
            Self::ATI => 3,
            Self::Intel => 4,
            Self::Unknown => 5,
        }
    }
}
impl Display for GpuType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LindberghColor {
    YELLOW,
    RED,
    BLUE,
    SILVER,
    REDEX,
}
impl Display for LindberghColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
#[derive(Debug, PartialEq, Eq)]
pub enum PrimevalHuntMode {
    NoTouchScreen = 1,
    SideBySide = 2,
    TouchScreenRight = 3,
    TouchScreenBottom = 4,
}
impl PrimevalHuntMode {
    pub fn into_i32(&self) -> i32 {
        match self {
            Self::NoTouchScreen => 1,
            Self::SideBySide => 2,
            Self::TouchScreenRight => 3,
            Self::TouchScreenBottom => 4,
        }
    }
}
// WARNING: Do not directly use this type
pub struct _Keymap<T> {
    // NOTE: By defualt,we follow Player1's test_key
    pub test: Option<T>,
    pub service: T,
    pub start: T,
    pub coin: Option<T>,
    pub up: T,
    pub down: T,
    pub left: T,
    pub right: T,
    pub button1: T,
    pub button2: T,
    pub button3: T,
    pub button4: T,
    pub button5: Option<T>,
    pub button6: Option<T>,
    pub button7: Option<T>,
    pub button8: Option<T>,
}

pub type SdlKeymap = _Keymap<egui::Key>;
pub type EvdevKeymap = _Keymap<String>;

pub struct _EvdevInput {
    pub player1: EvdevKeymap,
    pub player2: EvdevKeymap,
    pub analogues: [String; 4],
    pub analogue_deadzones: [(u32, u32, u32); 8],
}
impl _EvdevInput {
    pub fn read_from_lindbergh_conf(&mut self, buf: &String) -> anyhow::Result<()> {
        for (cnt, i) in buf.lines().enumerate() {
            let r = i.split_whitespace().collect::<Vec<&str>>();
            if r[0] == "#" || r.len() == 0 {
                continue;
            }
            if r.len() <= 1 {
                return Err(anyhow!("Too few arguments on line {}", cnt + 1));
            }
            match r[0] {
                "TEST_BUTTON" => {
                    self.player1.test = Some(r[1].to_string());
                }
                "PLAYER_1_BUTTON_START" => {
                    self.player1.start = r[1].to_string();
                }
                "PLAYER_1_BUTTON_SERVICE" => {
                    self.player1.service = r[1].to_string();
                }
                "PLAYER_1_BUTTON_UP" => {
                    self.player1.up = r[1].to_string();
                }
                "PLAYER_1_BUTTON_DOWN" => {
                    self.player1.down = r[1].to_string();
                }
                "PLAYER_1_BUTTON_LEFT" => {
                    self.player1.left = r[1].to_string();
                }
                "PLAYER_1_BUTTON_RIGHT" => {
                    self.player1.right = r[1].to_string();
                }
                "PLAYER_1_BUTTON_1" => {
                    self.player1.button1 = r[1].to_string();
                }
                "PLAYER_1_BUTTON_2" => {
                    self.player1.button2 = r[1].to_string();
                }
                "PLAYER_1_BUTTON_3" => {
                    self.player1.button3 = r[1].to_string();
                }
                "PLAYER_1_BUTTON_4" => {
                    self.player1.button4 = r[1].to_string();
                }
                "PLAYER_1_BUTTON_5" => {
                    self.player1.button5 = Some(r[1].to_string());
                }
                "PLAYER_1_BUTTON_6" => {
                    self.player1.button6 = Some(r[1].to_string());
                }
                "PLAYER_1_BUTTON_7" => {
                    self.player1.button7 = Some(r[1].to_string());
                }
                "PLAYER_1_BUTTON_8" => {
                    self.player1.button8 = Some(r[1].to_string());
                }
                "PLAYER_2_BUTTON_START" => {
                    self.player2.start = r[1].to_string();
                }
                "PLAYER_2_BUTTON_SERVICE" => {
                    self.player2.service = r[1].to_string();
                }
                "PLAYER_2_BUTTON_UP" => {
                    self.player2.up = r[1].to_string();
                }
                "PLAYER_2_BUTTON_DOWN" => {
                    self.player2.down = r[1].to_string();
                }
                "PLAYER_2_BUTTON_LEFT" => {
                    self.player2.left = r[1].to_string();
                }
                "PLAYER_2_BUTTON_RIGHT" => {
                    self.player2.right = r[1].to_string();
                }
                "PLAYER_2_BUTTON_1" => {
                    self.player2.button1 = r[1].to_string();
                }
                "PLAYER_2_BUTTON_2" => {
                    self.player2.button2 = r[1].to_string();
                }
                "PLAYER_2_BUTTON_3" => {
                    self.player2.button3 = r[1].to_string();
                }
                "PLAYER_2_BUTTON_4" => {
                    self.player2.button4 = r[1].to_string();
                }
                "PLAYER_2_BUTTON_5" => {
                    self.player2.button5 = Some(r[1].to_string());
                }
                "PLAYER_2_BUTTON_6" => {
                    self.player2.button6 = Some(r[1].to_string());
                }
                "PLAYER_2_BUTTON_7" => {
                    self.player2.button7 = Some(r[1].to_string());
                }
                "PLAYER_2_BUTTON_8" => {
                    self.player2.button8 = Some(r[1].to_string());
                }
                "ANALOGUE_1" => {
                    self.analogues[0] = r[1].to_string();
                }
                "ANALOGUE_2" => {
                    self.analogues[1] = r[1].to_string();
                }
                "ANALOGUE_3" => {
                    self.analogues[2] = r[1].to_string();
                }
                "ANALOGUE_4" => {
                    self.analogues[3] = r[1].to_string();
                }
                "ANALOGUE_DEADZONE_1" => {
                    if r.len() <= 4 {
                        return Err(anyhow!("Too few arguments on line {}", cnt + 1));
                    }
                    self.analogue_deadzones[0] = (r[1].parse()?, r[2].parse()?, r[3].parse()?);
                }
                "ANALOGUE_DEADZONE_2" => {
                    if r.len() <= 4 {
                        return Err(anyhow!("Too few arguments on line {}", cnt + 1));
                    }
                    self.analogue_deadzones[1] = (r[1].parse()?, r[2].parse()?, r[3].parse()?);
                }
                "ANALOGUE_DEADZONE_3" => {
                    if r.len() <= 4 {
                        return Err(anyhow!("Too few arguments on line {}", cnt + 1));
                    }
                    self.analogue_deadzones[2] = (r[1].parse()?, r[2].parse()?, r[3].parse()?);
                }
                "ANALOGUE_DEADZONE_4" => {
                    if r.len() <= 4 {
                        return Err(anyhow!("Too few arguments on line {}", cnt + 1));
                    }
                    self.analogue_deadzones[3] = (r[1].parse()?, r[2].parse()?, r[3].parse()?);
                }
                "ANALOGUE_DEADZONE_5" => {
                    if r.len() <= 4 {
                        return Err(anyhow!("Too few arguments on line {}", cnt + 1));
                    }
                    self.analogue_deadzones[4] = (r[1].parse()?, r[2].parse()?, r[3].parse()?);
                }
                "ANALOGUE_DEADZONE_6" => {
                    if r.len() <= 4 {
                        return Err(anyhow!("Too few arguments on line {}", cnt + 1));
                    }
                    self.analogue_deadzones[5] = (r[1].parse()?, r[2].parse()?, r[3].parse()?);
                }
                "ANALOGUE_DEADZONE_7" => {
                    if r.len() <= 4 {
                        return Err(anyhow!("Too few arguments on line {}", cnt + 1));
                    }
                    self.analogue_deadzones[6] = (r[1].parse()?, r[2].parse()?, r[3].parse()?);
                }
                "ANALOGUE_DEADZONE_8" => {
                    if r.len() <= 4 {
                        return Err(anyhow!("Too few arguments on line {}", cnt + 1));
                    }
                    self.analogue_deadzones[7] = (r[1].parse()?, r[2].parse()?, r[3].parse()?);
                }
                _ => {}
            }
        }
        Ok(())
    }
    pub fn write_to_lindbergh_conf(&self, f: &mut File) -> anyhow::Result<()> {
        if self.player1.test.is_none() && self.player2.test.is_none() {
            return Err(anyhow!("Unable to find test key"));
        }
        writeln!(
            f,
            "TEST_BUTTON {}",
            if self.player1.test.is_some() {
                self.player1.test.as_ref().unwrap()
            } else {
                self.player2.test.as_ref().unwrap()
            }
        )?;
        writeln!(f, "PLAYER_1_BUTTON_START {}", self.player1.start)?;
        writeln!(f, "PLAYER_1_BUTTON_SERVICE {}", self.player1.service)?;
        writeln!(f, "PLAYER_1_BUTTON_UP {}", self.player1.up)?;
        writeln!(f, "PLAYER_1_BUTTON_DOWN {}", self.player1.down)?;
        writeln!(f, "PLAYER_1_BUTTON_LEFT {}", self.player1.left)?;
        writeln!(f, "PLAYER_1_BUTTON_RIGHT {}", self.player1.right)?;
        writeln!(f, "PLAYER_1_BUTTON_1 {}", self.player1.button1)?;
        writeln!(f, "PLAYER_1_BUTTON_2 {}", self.player1.button2)?;
        writeln!(f, "PLAYER_1_BUTTON_3 {}", self.player1.button3)?;
        writeln!(f, "PLAYER_1_BUTTON_4 {}", self.player1.button4)?;
        writeln!(
            f,
            "PLAYER_1_BUTTON_5 {}",
            self.player1.button5.as_ref().unwrap()
        )?;
        writeln!(
            f,
            "PLAYER_1_BUTTON_6 {}",
            self.player1.button6.as_ref().unwrap()
        )?;
        writeln!(
            f,
            "PLAYER_1_BUTTON_7 {}",
            self.player1.button7.as_ref().unwrap()
        )?;
        writeln!(
            f,
            "PLAYER_1_BUTTON_8 {}",
            self.player1.button8.as_ref().unwrap()
        )?;

        writeln!(f, "PLAYER_2_BUTTON_START {}", self.player2.start)?;
        writeln!(f, "PLAYER_2_BUTTON_SERVICE {}", self.player2.service)?;
        writeln!(f, "PLAYER_2_BUTTON_UP {}", self.player2.up)?;
        writeln!(f, "PLAYER_2_BUTTON_DOWN {}", self.player2.down)?;
        writeln!(f, "PLAYER_2_BUTTON_LEFT {}", self.player2.left)?;
        writeln!(f, "PLAYER_2_BUTTON_RIGHT {}", self.player2.right)?;
        writeln!(f, "PLAYER_2_BUTTON_1 {}", self.player2.button1)?;
        writeln!(f, "PLAYER_2_BUTTON_2 {}", self.player2.button2)?;
        writeln!(f, "PLAYER_2_BUTTON_3 {}", self.player2.button3)?;
        writeln!(f, "PLAYER_2_BUTTON_4 {}", self.player2.button4)?;
        writeln!(
            f,
            "PLAYER_2_BUTTON_5 {}",
            self.player2.button5.as_ref().unwrap()
        )?;
        writeln!(
            f,
            "PLAYER_2_BUTTON_6 {}",
            self.player2.button6.as_ref().unwrap()
        )?;
        writeln!(
            f,
            "PLAYER_2_BUTTON_7 {}",
            self.player2.button7.as_ref().unwrap()
        )?;
        writeln!(
            f,
            "PLAYER_2_BUTTON_8 {}",
            self.player2.button8.as_ref().unwrap()
        )?;
        for (cnt, i) in (&self.analogues).iter().enumerate() {
            writeln!(f, "ANALOGUE_{} {}", cnt + 1, i)?;
        }
        for (cnt, i) in (&self.analogue_deadzones).iter().enumerate() {
            writeln!(f, "ANALOGUE_DEADZONE_{} {} {} {}", cnt + 1, i.0, i.1, i.2)?;
        }
        Ok(())
    }
}
impl SdlKeymap {
    pub fn read_from_lindbergh_conf(&mut self, buf: &String) -> anyhow::Result<()> {
        fn convert(s: String) -> anyhow::Result<egui::Key> {
            let r = s.parse::<u32>()?;
            egui_keycode_to_key(r).ok_or(anyhow!("Undefined Keycode {}",r))
        }
        for (cnt,i) in buf.lines().enumerate() {
            let r = i.split_whitespace().collect::<Vec<&str>>();
            if r[0] == "#" || r.len() == 0 {
                continue;
            }
            if r.len() <= 1 {
                return Err(anyhow!("Too few arguments on line {}", cnt + 1));
            }
            match r[0] {
                "TEST_KEY" => {
                    self.test = Some(convert(r[1].to_string())?);
                }
                "PLAYER_1_START_KEY" => {
                    self.start = convert(r[1].to_string())?;
                }
                "PLAYER_1_SERVICE_KEY" => {
                    self.service = convert(r[1].to_string())?;
                }
                "PLAYER_1_COIN_KEY" => {
                    self.coin = Some(convert(r[1].to_string())?);
                }
                "PLAYER_1_UP_KEY" => {
                    self.up = convert(r[1].to_string())?;
                }
                "PLAYER_1_DOWN_KEY" => {
                    self.down = convert(r[1].to_string())?;
                }
                "PLAYER_1_LEFT_KEY" => {
                    self.left = convert(r[1].to_string())?;
                }
                "PLAYER_1_RIGHT_KEY" => {
                    self.right = convert(r[1].to_string())?;
                }
                "PLAYER_1_BUTTON_1_KEY" => {
                    self.button1 = convert(r[1].to_string())?;
                }
                "PLAYER_1_BUTTON_2_KEY" => {
                    self.button2 = convert(r[1].to_string())?;
                }
                "PLAYER_1_BUTTON_3_KEY" => {
                    self.button3 = convert(r[1].to_string())?;
                }
                "PLAYER_1_BUTTON_4_KEY" => {
                    self.button4 = convert(r[1].to_string())?;
                }
                _ => {}
            }
        }
        Ok(())
    }
    pub fn write_to_lindbergh_conf(&self, f: &mut File) -> anyhow::Result<()> {
        if self.test.is_none() {
            return Err(anyhow!("Cannot find test key"));
        }
        if self.coin.is_none() {
            return Err(anyhow!("Cannot find coin key"));
        }
        fn result_key_to_keycode(key: &egui::Key) -> anyhow::Result<u32> {
            egui_key_to_keycode(key).ok_or(anyhow!("Cannot find corresponding keycode to key!"))
        }
        writeln!(
            f,
            "TEST_KEY {}",
            result_key_to_keycode(self.test.as_ref().unwrap())?
        )?;
        writeln!(
            f,
            "PLAYER_1_START_KEY {}",
            result_key_to_keycode(&self.start)?
        )?;
        writeln!(
            f,
            "PLAYER_1_SERVICE_KEY {}",
            result_key_to_keycode(&self.service)?
        )?;
        writeln!(
            f,
            "PLAYER_1_COIN_KEY {}",
            result_key_to_keycode(self.coin.as_ref().unwrap())?
        )?;
        writeln!(f, "PLAYER_1_UP_KEY {}", result_key_to_keycode(&self.up)?)?;
        writeln!(
            f,
            "PLAYER_1_DOWN_KEY {}",
            result_key_to_keycode(&self.down)?
        )?;
        writeln!(
            f,
            "PLAYER_1_LEFT_KEY {}",
            result_key_to_keycode(&self.left)?
        )?;
        writeln!(
            f,
            "PLAYER_1_RIGHT_KEY {}",
            result_key_to_keycode(&self.right)?
        )?;
        writeln!(
            f,
            "PLAYER_1_BUTTON_1_KEY {}",
            result_key_to_keycode(&self.button1)?
        )?;
        writeln!(
            f,
            "PLAYER_1_BUTTON_2_KEY {}",
            result_key_to_keycode(&self.button2)?
        )?;
        writeln!(
            f,
            "PLAYER_1_BUTTON_3_KEY {}",
            result_key_to_keycode(&self.button3)?
        )?;
        writeln!(
            f,
            "PLAYER_1_BUTTON_4_KEY {}",
            result_key_to_keycode(&self.button4)?
        )?;
        Ok(())
    }
}
// NOTE: Use this in other module only
pub enum Keymap {
    // SDL/X11 input does not support second player
    Sdl(_Keymap<egui::Key>),
    Evdev(_EvdevInput),
    Both(_Keymap<egui::Key>, _EvdevInput),
}
impl Keymap {
    fn into_i32(&self) -> i32 {
        match self {
            Self::Sdl(_) => 1,
            Self::Evdev(_) => 2,
            Self::Both(_, _) => 0,
        }
    }
    fn write_to_lindbergh_conf(&self, f: &mut File) -> anyhow::Result<()> {
        match self {
            Self::Sdl(s) => s.write_to_lindbergh_conf(f)?,
            Self::Evdev(e) => e.write_to_lindbergh_conf(f)?,
            Self::Both(s, e) => {
                s.write_to_lindbergh_conf(f)?;
                e.write_to_lindbergh_conf(f)?;
            }
        }
        Ok(())
    }
}
impl Default for Keymap {
    fn default() -> Self {
        Keymap::Both(SdlKeymap::default(), _EvdevInput::default())
    }
}
impl Default for SdlKeymap {
    fn default() -> Self {
        Self {
            test: Some(egui::Key::T),
            service: egui::Key::S,
            start: egui::Key::Num1,
            coin: Some(egui::Key::Num5),
            up: egui::Key::ArrowUp,
            down: egui::Key::ArrowDown,
            left: egui::Key::ArrowLeft,
            right: egui::Key::ArrowRight,
            button1: egui::Key::Q,
            button2: egui::Key::W,
            button3: egui::Key::E,
            button4: egui::Key::R,
            button5: None,
            button6: None,
            button7: None,
            button8: None,
        }
    }
}
impl Default for EvdevKeymap {
    fn default() -> Self {
        Self {
            test: Some("AT_TRANSLATED_SET_2_KEYBOARD_KEY_1".into()),
            start: "AT_TRANSLATED_SET_2_KEYBOARD_KEY_1".into(),
            coin: None,
            service: "AT_TRANSLATED_SET_2_KEYBOARD_KEY_S".into(),
            up: "AT_TRANSLATED_SET_2_KEYBOARD_KEY_UP".into(),
            down: "AT_TRANSLATED_SET_2_KEYBOARD_KEY_DOWN".into(),
            left: "AT_TRANSLATED_SET_2_KEYBOARD_KEY_LEFT".into(),
            right: "AT_TRANSLATED_SET_2_KEYBOARD_KEY_RIGHT".into(),
            button1: "AT_TRANSLATED_SET_2_KEYBOARD_KEY_Q".into(),
            button2: "AT_TRANSLATED_SET_2_KEYBOARD_KEY_W".into(),
            button3: "AT_TRANSLATED_SET_2_KEYBOARD_KEY_R".into(),
            button4: "AT_TRANSLATED_SET_2_KEYBOARD_KEY_T".into(),
            button5: Some("AT_TRANSLATED_SET_2_KEYBOARD_KEY_Y".into()),
            button6: Some("AT_TRANSLATED_SET_2_KEYBOARD_KEY_U".into()),
            button7: Some("AT_TRANSLATED_SET_2_KEYBOARD_KEY_I".into()),
            button8: Some("AT_TRANSLATED_SET_2_KEYBOARD_KEY_O".into()),
        }
    }
}
impl Default for _EvdevInput {
    fn default() -> Self {
        Self {
            analogues: [
                "SYNPS_2_SYNAPTICS_TOUCHPAD_ABS_X".into(),
                "SYNPS_2_SYNAPTICS_TOUCHPAD_ABS_Y".into(),
                "SYNPS_2_SYNAPTICS_TOUCHPAD_ABS_Z".into(),
                "SYNPS_2_SYNAPTICS_TOUCHPAD_ABS_RZ".into(),
            ],
            analogue_deadzones: [(0, 0, 0); 8],
            player1: EvdevKeymap::default(),
            player2: EvdevKeymap::default(),
        }
    }
}
/**
 * The default value should be same as docs/lindbergh.conf
 * if game type is driving:
 *      serial_port1 -> driveboard
 * if game type is special (such as hod4sp):
 *      serial_port1 -> rideboard
 * if game type is OUTRUN 2 SP SDX
 *      serial_port1 -> driveboard
 *      serial_port2 -> motionboard
 */
pub struct LindberghConfig {
    pub exe_path: String,
    // width,height
    pub window_size: (u32, u32),
    pub fullscreen: bool,
    pub disable_sdl: bool,
    pub game_region: GameRegion,
    pub freeplay: bool,
    pub input_method: Keymap,
    pub jvs_path: String,
    pub emulate_jvs: bool,
    pub serial_port1: String,
    pub emulate_rideboard: bool,
    pub emulate_motionboard: bool,
    pub emulate_driveboard: bool,
    pub serial_port2: String,
    pub sram_path: String,
    pub eeprom_path: String,
    pub gpu_vendor: GpuType,
    pub debug_message: bool,
    pub hammer_flicker_fix: bool,
    pub keep_aspect_ratio: bool,
    pub outrun_lens_glare_enable: bool,
    pub enable_fps_limiter: bool,
    pub limit_fps_target: u32,
    pub border_enabled: bool,
    pub white_border_percentage: u32,
    pub black_border_percentage: u32,
    pub lets_go_jungle_render_with_mesa: bool,
    pub skip_outrun_cabinet_check: bool,
    pub mj4_enable_all_time: bool,
    pub primevalhunt_mode: Option<PrimevalHuntMode>,
    pub lindbergh_color: LindberghColor,
}

impl Default for LindberghConfig {
    fn default() -> Self {
        Self {
            exe_path: String::new(),
            window_size: (640, 480),
            fullscreen: false,
            freeplay: false,
            disable_sdl: false,
            input_method: Keymap::default(),
            game_region: GameRegion::JP,
            jvs_path: "/dev/ttyUSB0".into(),
            emulate_jvs: true,
            serial_port1: "/dev/ttyS0".into(),
            serial_port2: "/dev/ttyS1".into(),
            emulate_motionboard: true,
            emulate_rideboard: true,
            emulate_driveboard: true,
            sram_path: "sram.bin".into(),
            eeprom_path: "eeprom.bin".into(),
            gpu_vendor: GpuType::AutoDetect,
            debug_message: false,
            hammer_flicker_fix: false,
            keep_aspect_ratio: false,
            outrun_lens_glare_enable: false,
            enable_fps_limiter: false,
            limit_fps_target: 60,
            border_enabled: false,
            white_border_percentage: 2,
            black_border_percentage: 0,
            lets_go_jungle_render_with_mesa: true,
            skip_outrun_cabinet_check: false,
            mj4_enable_all_time: true,
            primevalhunt_mode: None,
            lindbergh_color: LindberghColor::YELLOW,
        }
    }
}
impl LindberghConfig {
    pub fn from_lindbergh_conf(&mut self, conf_path: impl AsRef<Path>) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn write_to_lindbergh_conf(
        &self,
        conf_path: impl AsRef<Path>,
        current_title: &GameTitle,
    ) -> anyhow::Result<()> {
        if !fs::exists(&conf_path)? {
            File::create_new(&conf_path)?;
        }
        let mut f = File::options().write(true).open(conf_path)?;
        let mut e = File::options()
            .append(true)
            .open("./config/exe_paths.conf")?;
        writeln!(e, "{:?} {}", current_title, self.exe_path)?;
        writeln!(f, "# This file is generated by lindbergh-loader-gui")?;
        writeln!(
            f,
            "# Do not make any changes unless you know what you're doing"
        )?;
        writeln!(f, "WIDTH {}", self.window_size.0)?;
        writeln!(f, "HEIGHT {}", self.window_size.1)?;
        writeln!(f, "FULLSCREEN {}", self.fullscreen as i32)?;
        writeln!(f, "INPUT_MODE {}", self.input_method.into_i32())?;
        writeln!(f, "NO_SDL {}", self.disable_sdl as i32)?;
        writeln!(f, "REGION {}", self.game_region)?;
        writeln!(f, "FREEPLAY {}", self.freeplay)?;
        writeln!(f, "EMULATE_JVS {}", self.emulate_jvs)?;
        writeln!(f, "EMULATE_RIDEBOARD {}", self.emulate_rideboard as i32)?;
        writeln!(f, "EMULATE_MOTIONBOARD {}", self.emulate_motionboard as i32)?;
        writeln!(f, "EMULATE_DRIVEBOARD {}", self.emulate_driveboard as i32)?;
        writeln!(f, "JVS_PATH {}", self.jvs_path)?;
        writeln!(f, "SERIAL_1_PATH {}", self.serial_port1)?;
        writeln!(f, "SERIAL_2_PATH {}", self.serial_port2)?;
        writeln!(f, "SRAM_PATH {}", self.sram_path)?;
        writeln!(f, "EEPROM_PATH {}", self.eeprom_path)?;
        writeln!(f, "GPU_VENDOR {}", self.gpu_vendor.into_i32())?;
        writeln!(f, "DEBUG_MSGS {}", self.debug_message as i32)?;
        writeln!(f, "BORDER_ENABLED {}", self.border_enabled as i32)?;
        writeln!(
            f,
            "WHITE_BORDER_PERCENTAGE {}",
            self.white_border_percentage
        )?;
        writeln!(
            f,
            "BLACK_BORDER_PERCENTAGE {}",
            self.black_border_percentage
        )?;
        writeln!(f, "HUMMER_FLICKER_FIX {}", self.hammer_flicker_fix)?;
        writeln!(f, "KEEP_ASPECT_RATIO {}", self.keep_aspect_ratio)?;
        writeln!(
            f,
            "OUTRUN_LENS_GLARE_ENABLED {}",
            self.outrun_lens_glare_enable
        )?;
        writeln!(
            f,
            "SKIP_OUTRUN_CABINET_CHECK {}",
            self.skip_outrun_cabinet_check
        )?;
        writeln!(f, "FPS_LIMITER_ENABLED {}", self.enable_fps_limiter)?;
        writeln!(f, "FPS_TARGET {}", self.limit_fps_target)?;
        writeln!(
            f,
            "LGJ_RENDER_WITH_MESA {}",
            self.lets_go_jungle_render_with_mesa
        )?;
        writeln!(
            f,
            "PRIMEVAL_HUNT_MODE {}",
            if self.primevalhunt_mode.is_none() {
                0
            } else {
                self.primevalhunt_mode.as_ref().unwrap().into_i32()
            }
        )?;
        writeln!(f, "LINDBERGH_COLOUR {}", self.lindbergh_color)?;
        self.input_method.write_to_lindbergh_conf(&mut f)?;
        Ok(())
    }
}
