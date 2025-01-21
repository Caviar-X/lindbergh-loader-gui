use crate::{
    games::GameTitle,
    ui::{egui_key_to_keycode, egui_keycode_to_key},
};
use anyhow::{Ok, anyhow};
use eframe::egui;
use std::{
    fmt::{Debug, Display},
    fs::{self, File, read_to_string},
    io::Write,
    path::Path,
};

fn i32_to_bool(value: i32) -> Option<bool> {
    match value {
        0 => Some(false),
        1 => Some(true),
        _ => None,
    }
}

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
    fn as_i32(&self) -> i32 {
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
#[derive(Clone, PartialEq)]
pub struct _Keymap<T: Clone + PartialEq> {
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

#[derive(Clone, PartialEq)]
pub struct _EvdevInput {
    pub player1: EvdevKeymap,
    pub player2: EvdevKeymap,
    pub analogues: [String; 4],
    pub analogue_deadzones: [(u32, u32, u32); 8],
}
impl _EvdevInput {
    pub fn read_from_lindbergh_conf(&mut self, buf: &str) -> anyhow::Result<()> {
        for (cnt, i) in buf.lines().enumerate() {
            let r = i.split_whitespace().collect::<Vec<&str>>();
            if r[0] == "#" || r.is_empty() {
                continue;
            }
            if r.len() < 2 {
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
                    if r.len() < 4 {
                        return Err(anyhow!("Too few arguments on line {}", cnt + 1));
                    }
                    self.analogue_deadzones[0] = (r[1].parse()?, r[2].parse()?, r[3].parse()?);
                }
                "ANALOGUE_DEADZONE_2" => {
                    if r.len() < 4 {
                        return Err(anyhow!("Too few arguments on line {}", cnt + 1));
                    }
                    self.analogue_deadzones[1] = (r[1].parse()?, r[2].parse()?, r[3].parse()?);
                }
                "ANALOGUE_DEADZONE_3" => {
                    if r.len() < 4 {
                        return Err(anyhow!("Too few arguments on line {}", cnt + 1));
                    }
                    self.analogue_deadzones[2] = (r[1].parse()?, r[2].parse()?, r[3].parse()?);
                }
                "ANALOGUE_DEADZONE_4" => {
                    if r.len() < 4 {
                        return Err(anyhow!("Too few arguments on line {}", cnt + 1));
                    }
                    self.analogue_deadzones[3] = (r[1].parse()?, r[2].parse()?, r[3].parse()?);
                }
                "ANALOGUE_DEADZONE_5" => {
                    if r.len() < 4 {
                        return Err(anyhow!("Too few arguments on line {}", cnt + 1));
                    }
                    self.analogue_deadzones[4] = (r[1].parse()?, r[2].parse()?, r[3].parse()?);
                }
                "ANALOGUE_DEADZONE_6" => {
                    if r.len() < 4 {
                        return Err(anyhow!("Too few arguments on line {}", cnt + 1));
                    }
                    self.analogue_deadzones[5] = (r[1].parse()?, r[2].parse()?, r[3].parse()?);
                }
                "ANALOGUE_DEADZONE_7" => {
                    if r.len() < 4 {
                        return Err(anyhow!("Too few arguments on line {}", cnt + 1));
                    }
                    self.analogue_deadzones[6] = (r[1].parse()?, r[2].parse()?, r[3].parse()?);
                }
                "ANALOGUE_DEADZONE_8" => {
                    if r.len() < 4 {
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
        for (cnt, i) in self.analogues.iter().enumerate() {
            writeln!(f, "ANALOGUE_{} {}", cnt + 1, i)?;
        }
        for (cnt, i) in self.analogue_deadzones.iter().enumerate() {
            writeln!(f, "ANALOGUE_DEADZONE_{} {} {} {}", cnt + 1, i.0, i.1, i.2)?;
        }
        Ok(())
    }
}
impl SdlKeymap {
    pub fn read_from_lindbergh_conf(&mut self, buf: &str) -> anyhow::Result<()> {
        fn result_keycode_to_key(s: &str) -> anyhow::Result<egui::Key> {
            let r = s.parse::<u32>()?;
            egui_keycode_to_key(r).ok_or(anyhow!("Undefined Keycode {}", r))
        }
        for (cnt, i) in buf.lines().enumerate() {
            let r = i.split_whitespace().collect::<Vec<&str>>();
            if r[0] == "#" || r.is_empty() {
                continue;
            }
            if r.len() < 2 {
                return Err(anyhow!("Too few arguments on line {}", cnt + 1));
            }
            match r[0] {
                "TEST_KEY" => {
                    self.test = Some(result_keycode_to_key(r[1])?);
                }
                "PLAYER_1_START_KEY" => {
                    self.start = result_keycode_to_key(r[1])?;
                }
                "PLAYER_1_SERVICE_KEY" => {
                    self.service = result_keycode_to_key(r[1])?;
                }
                "PLAYER_1_COIN_KEY" => {
                    self.coin = Some(result_keycode_to_key(r[1])?);
                }
                "PLAYER_1_UP_KEY" => {
                    self.up = result_keycode_to_key(r[1])?;
                }
                "PLAYER_1_DOWN_KEY" => {
                    self.down = result_keycode_to_key(r[1])?;
                }
                "PLAYER_1_LEFT_KEY" => {
                    self.left = result_keycode_to_key(r[1])?;
                }
                "PLAYER_1_RIGHT_KEY" => {
                    self.right = result_keycode_to_key(r[1])?;
                }
                "PLAYER_1_BUTTON_1_KEY" => {
                    self.button1 = result_keycode_to_key(r[1])?;
                }
                "PLAYER_1_BUTTON_2_KEY" => {
                    self.button2 = result_keycode_to_key(r[1])?;
                }
                "PLAYER_1_BUTTON_3_KEY" => {
                    self.button3 = result_keycode_to_key(r[1])?;
                }
                "PLAYER_1_BUTTON_4_KEY" => {
                    self.button4 = result_keycode_to_key(r[1])?;
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
#[derive(Clone, PartialEq)]
pub enum Keymap {
    // SDL/X11 input does not support second player by far
    // NOTE: This should be changed when lindbergh-loader supports two players
    Sdl(SdlKeymap),
    Evdev(_EvdevInput),
    Both(SdlKeymap, _EvdevInput),
}
impl Debug for Keymap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sdl(_) => write!(f, "SDL/X11"),
            Self::Evdev(_) => write!(f, "Evdev"),
            Self::Both(_, _) => write!(f, "Both"),
        }
    }
}
impl Keymap {
    fn as_i32(&self) -> i32 {
        match self {
            Self::Sdl(_) => 1,
            Self::Evdev(_) => 2,
            Self::Both(_, _) => 0,
        }
    }
    // use this before check
    pub fn get_sdlkeymap(&self) -> Option<&SdlKeymap> {
        match self {
            Self::Sdl(s) => Some(s),
            Self::Both(s, _) => Some(s),
            _ => None,
        }
    }
    pub fn get_sdlkeymap_mut(&mut self) -> Option<&mut SdlKeymap> {
        match self {
            Self::Sdl(s) => Some(s),
            Self::Both(s, _) => Some(s),
            _ => None,
        }
    }
    pub fn get_evdev(&self) -> Option<&_EvdevInput> {
        match self {
            Self::Both(_, e) => Some(e),
            Self::Evdev(e) => Some(e),
            _ => None,
        }
    }
    pub fn get_evdev_mut(&mut self) -> Option<&mut _EvdevInput> {
        match self {
            Self::Both(_, e) => Some(e),
            Self::Evdev(e) => Some(e),
            _ => None,
        }
    }
    pub fn into_both(self) -> Self {
        match self {
            Self::Sdl(s) => Self::Both(s, _EvdevInput::default()),
            Self::Evdev(e) => Self::Both(SdlKeymap::default(), e),
            Self::Both(_, _) => self,
        }
    }
    pub fn into_sdl(self) -> Self {
        match self {
            Self::Sdl(_) => self,
            Self::Evdev(_) => Self::Sdl(SdlKeymap::default()),
            Self::Both(s, _) => Self::Sdl(s),
        }
    }
    pub fn into_evdev(self) -> Self {
        match self {
            Self::Sdl(_) => Self::Evdev(_EvdevInput::default()),
            Self::Evdev(_) => self,
            Self::Both(_, e) => Self::Evdev(e),
        }
    }
    pub fn has_both(&self) -> bool {
        matches!(self, Keymap::Both(_, _))
    }
    pub fn has_sdl(&self) -> bool {
        matches!(self, Keymap::Sdl(_)) || self.has_both()
    }
    pub fn has_evdev(&self) -> bool {
        matches!(self, Keymap::Evdev(_)) || self.has_both()
    }
    pub fn write_to_lindbergh_conf(&self, f: &mut File) -> anyhow::Result<()> {
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
    pub fn read_from_lindbergh_conf(&mut self, buf: &str) -> anyhow::Result<()> {
        for (cnt, i) in buf.lines().enumerate() {
            let r = i.split_whitespace().collect::<Vec<&str>>();
            if r[0] == "#" || r.is_empty() {
                continue;
            }
            if r.len() < 2 {
                return Err(anyhow!("Too few arguments on line {}", cnt + 1));
            }
            if r[0] == "INPUT_MODE" {
                match r[1].parse::<u32>()? {
                    0 => {
                        let mut s = SdlKeymap::default();
                        s.read_from_lindbergh_conf(buf)?;
                        let mut e = _EvdevInput::default();
                        e.read_from_lindbergh_conf(buf)?;
                        *self = Self::Both(s, e);
                    }
                    1 => {
                        let mut s = SdlKeymap::default();
                        s.read_from_lindbergh_conf(buf)?;
                        *self = Self::Sdl(s);
                    }
                    2 => {
                        let mut e = _EvdevInput::default();
                        e.read_from_lindbergh_conf(buf)?;
                        *self = Self::Evdev(e);
                    }
                    _ => {
                        return Err(anyhow!("Invaild input mode"));
                    }
                }
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
    pub primevalhunt_mode: PrimevalHuntMode,
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
            primevalhunt_mode: PrimevalHuntMode::NoTouchScreen,
            lindbergh_color: LindberghColor::YELLOW,
        }
    }
}
impl LindberghConfig {
    pub fn write_to_lindbergh_conf(&self, current_title: &GameTitle) -> anyhow::Result<()> {
        let path = format!("./config/{:?}.conf", current_title);
        if !fs::exists(&path)? {
            File::create_new(&path)?;
        }
        let mut f = File::options().write(true).truncate(true).open(&path)?;
        executable_path::add_exe_path(current_title, &self.exe_path)?;
        writeln!(f, "# This file is generated by lindbergh-loader-gui")?;
        writeln!(
            f,
            "# Do not make any changes unless you know what you're doing"
        )?;
        writeln!(f, "WIDTH {}", self.window_size.0)?;
        writeln!(f, "HEIGHT {}", self.window_size.1)?;
        writeln!(f, "FULLSCREEN {}", self.fullscreen as i32)?;
        writeln!(f, "INPUT_MODE {}", self.input_method.as_i32())?;
        writeln!(f, "NO_SDL {}", self.disable_sdl as i32)?;
        writeln!(f, "REGION {}", self.game_region)?;
        writeln!(f, "FREEPLAY {}", self.freeplay as i32)?;
        writeln!(f, "EMULATE_JVS {}", self.emulate_jvs as i32)?;
        writeln!(f, "EMULATE_RIDEBOARD {}", self.emulate_rideboard as i32)?;
        writeln!(f, "EMULATE_MOTIONBOARD {}", self.emulate_motionboard as i32)?;
        writeln!(f, "EMULATE_DRIVEBOARD {}", self.emulate_driveboard as i32)?;
        writeln!(f, "JVS_PATH {}", self.jvs_path)?;
        writeln!(f, "SERIAL_1_PATH {}", self.serial_port1)?;
        writeln!(f, "SERIAL_2_PATH {}", self.serial_port2)?;
        writeln!(f, "SRAM_PATH {}", self.sram_path)?;
        writeln!(f, "EEPROM_PATH {}", self.eeprom_path)?;
        writeln!(f, "GPU_VENDOR {}", self.gpu_vendor.as_i32())?;
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
        writeln!(f, "HUMMER_FLICKER_FIX {}", self.hammer_flicker_fix as i32)?;
        writeln!(f, "KEEP_ASPECT_RATIO {}", self.keep_aspect_ratio as i32)?;
        writeln!(
            f,
            "OUTRUN_LENS_GLARE_ENABLED {}",
            self.outrun_lens_glare_enable as i32
        )?;
        writeln!(
            f,
            "SKIP_OUTRUN_CABINET_CHECK {}",
            self.skip_outrun_cabinet_check as i32
        )?;
        writeln!(f, "FPS_LIMITER_ENABLED {}", self.enable_fps_limiter as i32)?;
        writeln!(f, "FPS_TARGET {}", self.limit_fps_target)?;
        writeln!(
            f,
            "LGJ_RENDER_WITH_MESA {}",
            self.lets_go_jungle_render_with_mesa as i32
        )?;
        writeln!(
            f,
            "PRIMEVAL_HUNT_MODE {}",
            self.primevalhunt_mode.into_i32()
        )?;
        writeln!(f, "LINDBERGH_COLOUR {}", self.lindbergh_color)?;
        writeln!(
            f,
            "MJ4_ENABLED_ALL_THE_TIME {}",
            self.mj4_enable_all_time as i32
        )?;
        self.input_method.write_to_lindbergh_conf(&mut f)?;
        Ok(())
    }
    pub fn read_from_lindbergh_conf(&mut self, buf: &str) -> anyhow::Result<()> {
        fn result_i32_to_bool(value: i32, line: usize) -> anyhow::Result<bool> {
            i32_to_bool(value).ok_or(anyhow!(
                "Invaild value at line {},should be only 1 or 0",
                line
            ))
        }
        self.input_method.read_from_lindbergh_conf(buf)?;
        for (cnt, i) in buf.lines().enumerate() {
            let r = i.split_whitespace().collect::<Vec<&str>>();
            if r[0] == "#" || r.is_empty() {
                continue;
            }
            if r.len() < 2 {
                return Err(anyhow!("Too few arguments on line {}", cnt + 1));
            }
            match r[0] {
                "WIDTH" => {
                    if r[1] == "AUTO" {
                        continue;
                    }
                    self.window_size.0 = r[1].parse()?;
                }
                "HEIGHT" => {
                    if r[1] == "AUTO" {
                        continue;
                    }
                    self.window_size.1 = r[1].parse()?;
                }
                "FULLSCREEN" => {
                    self.fullscreen = result_i32_to_bool(r[1].parse()?, cnt + 1)?;
                }
                "NO_SDL" => {
                    self.disable_sdl = result_i32_to_bool(r[1].parse()?, cnt + 1)?;
                }
                "REGION" => match r[1] {
                    "JP" => self.game_region = GameRegion::JP,
                    "US" => self.game_region = GameRegion::US,
                    "EX" => self.game_region = GameRegion::EX,
                    _ => {
                        return Err(anyhow!("Invaild game region {}", r[1]));
                    }
                },
                "FREEPLAY" => {
                    self.freeplay = result_i32_to_bool(r[1].parse()?, cnt + 1)?;
                }
                "EMULATE_JVS" => {
                    self.emulate_jvs = result_i32_to_bool(r[1].parse()?, cnt + 1)?;
                }
                "EMULATE_RIDEBOARD" => {
                    if r[1] == "AUTO" {
                        continue;
                    }
                    self.emulate_rideboard = result_i32_to_bool(r[1].parse()?, cnt + 1)?;
                }
                "EMULATE_DRIVEBOARD" => {
                    if r[1] == "AUTO" {
                        continue;
                    }
                    self.emulate_driveboard = result_i32_to_bool(r[1].parse()?, cnt + 1)?;
                }
                "EMULATE_MOTIONBOARD" => {
                    if r[1] == "AUTO" {
                        continue;
                    }
                    self.emulate_motionboard = result_i32_to_bool(r[1].parse()?, cnt + 1)?;
                }
                "JVS_PATH" => {
                    self.jvs_path = r[1].to_string();
                }
                "SERIAL_1_PATH" => {
                    self.serial_port1 = r[1].to_string();
                }
                "SERIAL_2_PATH" => {
                    self.serial_port2 = r[1].to_string();
                }
                "SRAM_PATH" => {
                    self.sram_path = r[1].to_string();
                }
                "EEPROM_PATH" => {
                    self.eeprom_path = r[1].to_string();
                }
                "GPU_VENDOR" => match r[1].parse::<u32>()? {
                    0 => {
                        self.gpu_vendor = GpuType::AutoDetect;
                    }
                    1 => {
                        self.gpu_vendor = GpuType::Nvidia;
                    }
                    2 => {
                        self.gpu_vendor = GpuType::AMD;
                    }
                    3 => {
                        self.gpu_vendor = GpuType::ATI;
                    }
                    4 => {
                        self.gpu_vendor = GpuType::Intel;
                    }
                    5 => {
                        self.gpu_vendor = GpuType::Unknown;
                    }
                    _ => {
                        return Err(anyhow!("Invaild GPU vendor"));
                    }
                },
                "DEBUG_MSGS" => {
                    self.debug_message = result_i32_to_bool(r[1].parse()?, cnt + 1)?;
                }
                "BORDER_ENABLED" => {
                    self.border_enabled = result_i32_to_bool(r[1].parse()?, cnt + 1)?;
                }
                "WHITE_BORDER_PERCENTAGE" => {
                    let p = r[1].parse::<u32>()?;
                    if p > 100 {
                        return Err(anyhow!(
                            "WHITE_BORDER_PERCENTAGE should be a value from 0 to 100"
                        ));
                    }
                    self.white_border_percentage = p;
                }
                "BLACK_BORDER_PERCENTAGE" => {
                    let p = r[1].parse::<u32>()?;
                    if p > 100 {
                        return Err(anyhow!(
                            "BLACK_BORDER_PERCENTAGE should be a value from 0 to 100"
                        ));
                    }
                    self.black_border_percentage = p;
                }
                "HUMMER_FLICKER_FIX" => {
                    self.hammer_flicker_fix = result_i32_to_bool(r[1].parse()?, cnt + 1)?;
                }
                "KEEP_ASPECT_RATIO" => {
                    self.keep_aspect_ratio = result_i32_to_bool(r[1].parse()?, cnt + 1)?;
                }
                "OUTRUN_LENS_GLARE_ENABLED" => {
                    self.outrun_lens_glare_enable = result_i32_to_bool(r[1].parse()?, cnt + 1)?;
                }
                "SKIP_OUTRUN_CABINET_CHECK" => {
                    self.skip_outrun_cabinet_check = result_i32_to_bool(r[1].parse()?, cnt + 1)?;
                }
                "FPS_LIMITER_ENABLED" => {
                    self.enable_fps_limiter = result_i32_to_bool(r[1].parse()?, cnt + 1)?;
                }
                "FPS_TARGET" => {
                    self.limit_fps_target = r[1].parse()?;
                }
                "LGJ_RENDER_WITH_MESA" => {
                    self.lets_go_jungle_render_with_mesa =
                        result_i32_to_bool(r[1].parse()?, cnt + 1)?;
                }
                "PRIMEVAL_HUNT_MODE" => match r[1].parse::<u32>()? {
                    0 | 1 => {
                        self.primevalhunt_mode = PrimevalHuntMode::NoTouchScreen;
                    }
                    2 => {
                        self.primevalhunt_mode = PrimevalHuntMode::SideBySide;
                    }
                    3 => {
                        self.primevalhunt_mode = PrimevalHuntMode::TouchScreenRight;
                    }
                    4 => {
                        self.primevalhunt_mode = PrimevalHuntMode::TouchScreenBottom;
                    }
                    _ => {
                        return Err(anyhow!("Invaild primeval hunt mode"));
                    }
                },
                "LINDBERGH_COLOUR" => match r[1] {
                    "YELLOW" => {
                        self.lindbergh_color = LindberghColor::YELLOW;
                    }
                    "RED" => {
                        self.lindbergh_color = LindberghColor::RED;
                    }
                    "BLUE" => {
                        self.lindbergh_color = LindberghColor::BLUE;
                    }
                    "SILVER" => {
                        self.lindbergh_color = LindberghColor::SILVER;
                    }
                    "REDEX" => {
                        self.lindbergh_color = LindberghColor::REDEX;
                    }
                    _ => {
                        return Err(anyhow!("Invaild lindbergh color"));
                    }
                },
                "MJ4_ENABLED_ALL_THE_TIME" => {
                    self.mj4_enable_all_time = result_i32_to_bool(r[1].parse()?, cnt + 1)?;
                }
                _ => {}
            }
        }
        Ok(())
    }
    pub fn read_from_lindbergh_conf_by_path(
        &mut self,
        path: impl AsRef<Path>,
    ) -> anyhow::Result<()> {
        let buf = read_to_string(path)?;
        self.read_from_lindbergh_conf(&buf)?;
        Ok(())
    }
    pub fn read_from_lindbergh_conf_by_title(
        &mut self,
        current_title: &GameTitle,
    ) -> anyhow::Result<()> {
        // first we have to find exe_path
        for (cnt, i) in read_to_string("./config/exe_paths.conf")?
            .lines()
            .enumerate()
        {
            if i.starts_with("#") || i.trim().is_empty() {
                continue;
            }
            let (name, path) = i.split_once(char::is_whitespace).ok_or(anyhow!(
                "Too few arguments at ./config/exe_paths.conf line {}",
                cnt + 1
            ))?;
            if name == format!("{:?}", current_title) {
                self.exe_path = String::from(path);
                break;
            }
        }
        let path = format!("./config/{:?}.conf", current_title);
        self.read_from_lindbergh_conf_by_path(path)?;
        Ok(())
    }
}

pub mod executable_path {
    use crate::games::{GameData, GameTitle};
    use anyhow::*;
    use std::fs::{File, read_to_string};
    use std::io::Write;
    pub fn add_exe_path(current_title: &GameTitle, exe_path: impl ToString) -> anyhow::Result<()> {
        let buf = read_to_string("./config/exe_paths.conf")?;
        let mut e = File::options()
            .write(true)
            .open("./config/exe_paths.conf")?;
        if buf.lines().filter(|x| !x.starts_with("#")).count() == 0 {
            writeln!(e, "# This file is generated by lindbergh-loader-gui")?;
            writeln!(
                e,
                "# Do not make any changes unless you know what you're doing"
            )?;
            writeln!(e, "{:?} {}", current_title, exe_path.to_string())?;
        } else {
            for i in buf.lines() {
                if i.contains(&format!("{:?}", current_title)) {
                    writeln!(e, "{:?} {}", current_title, exe_path.to_string())?;
                } else {
                    writeln!(e, "{}", i)?;
                }
            }
        }
        Ok(())
    }
    pub fn remove_exe_path(current_title: &GameTitle) -> anyhow::Result<()> {
        let buf = read_to_string("./config/exe_paths.conf")?;
        let mut e = File::options()
            .truncate(true)
            .write(true)
            .open("./config/exe_paths.conf")?;
        for i in buf.lines() {
            if !i.contains(&format!("{:?}", current_title)) {
                writeln!(e, "{}", i)?;
            }
        }
        Ok(())
    }
    pub fn get_path(current_title: &GameTitle) -> anyhow::Result<String> {
        let buf = read_to_string("./config/exe_paths.conf")?;
        for (cnt, i) in buf.lines().enumerate() {
            let (name, path) = i.split_once(char::is_whitespace).ok_or(anyhow!(
                "Invaild executable path argument on line {}",
                cnt + 1
            ))?;
            if format!("{:?}", current_title) == name {
                return Ok(path.into());
            }
        }
        Err(anyhow!("Unable to find path"))
    }
    pub fn get_list() -> anyhow::Result<Vec<GameData>> {
        let buf = read_to_string("./config/exe_paths.conf")?;
        let mut game_library: Vec<GameData> = Vec::new();
        for (cnt, i) in buf.lines().enumerate() {
            if i.starts_with("#") || i.trim().is_empty() {
                continue;
            }

            let (name, _) = i.split_once(char::is_whitespace).ok_or(anyhow!(
                "Invaild executable path argument on line {}",
                cnt + 1
            ))?;
            for j in GameTitle::all_variants() {
                if format!("{:?}", j) == name {
                    game_library.push(j.as_gamedata());
                    break;
                }
            }
        }
        Ok(game_library)
    }
}
