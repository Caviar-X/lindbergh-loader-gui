use std::fmt::Display;

use eframe::egui;
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
impl Display for GpuType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
#[derive(Debug, PartialEq, Eq)]
pub enum LindberghColor {
    YELLOW,
    RED,
    BLUE,
    SILVER,
    REDEX,
}
#[derive(Debug, PartialEq, Eq)]
pub enum PrimevalHuntMode {
    NoTouchScreen = 1,
    SideBySide = 2,
    TouchScreenRight = 3,
    TouchScreenBottom = 4,
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
pub struct _EvdevInput {
    pub player1: _Keymap<String>,
    pub player2: _Keymap<String>,
    pub analogue1: String,
    pub analogue2: String,
    pub analogue3: String,
    pub analogue4: String,
}
pub type SdlKeymap = _Keymap<egui::Key>;
pub type EvdevKeymap = _Keymap<String>;

pub enum Keymap {
    // SDL/X11 input does not support second player
    Sdl(_Keymap<egui::Key>),
    Evdev(_EvdevInput),
    Both {
        sdl_input: _Keymap<egui::Key>,
        evdev_input: _EvdevInput,
    },
}
impl Default for Keymap {
    fn default() -> Self {
        Keymap::Both {
            sdl_input: SdlKeymap::default(),
            evdev_input: _EvdevInput::default(),
        }
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
            analogue1: "SYNPS_2_SYNAPTICS_TOUCHPAD_ABS_X".into(),
            analogue2: "SYNPS_2_SYNAPTICS_TOUCHPAD_ABS_Y".into(),
            analogue3: "SYNPS_2_SYNAPTICS_TOUCHPAD_ABS_Z".into(),
            analogue4: "SYNPS_2_SYNAPTICS_TOUCHPAD_ABS_RZ".into(),
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
    // Display width,height
    pub window_size: (u32, u32),
    pub fullscreen: bool,
    pub disable_sdl: bool,
    pub game_region: GameRegion,
    pub freeplay: bool,
    pub input_method: Keymap,
    // if this is none,jvs will be emulated
    pub jvs_path: String,
    pub emulate_jvs: bool,
    // if this is none,rideboard,driveboard or motionboard will be emulated
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
    pub lets_go_jungle_render_with_mesa: bool,
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
            lets_go_jungle_render_with_mesa: false,
            primevalhunt_mode: None,
            lindbergh_color: LindberghColor::YELLOW,
        }
    }
}
