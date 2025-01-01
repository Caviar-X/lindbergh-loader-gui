use eframe::egui;
use std::path::PathBuf;
#[derive(Debug)]
pub enum GameRegion {
    JP,
    US,
    EX
}
pub enum GpuType {
    AutoDetect = 0,
    Nvidia = 1,
    AMD = 2,
    ATI = 3,
    Intel = 4,
    Unknown = 5,
}
pub enum LindberghColor {
    YELLOW,
    RED,
    BLUE,
    SILVER,
    REDEX
}
pub enum PrimevalHuntMode {
    NoTouchScreen = 1,
    SideBySide = 2,
    TouchScreenRight = 3,
    TouchScreenBottom = 4
}
impl ToString for GameRegion {
    fn to_string(&self) -> String {
        format!("{:?}",self)
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
pub struct _EvdevInput {
    pub player1: _Keymap<String>,
    pub player2: _Keymap<String>,
    pub analogue1: String,
    pub analogue2: String,
    pub analogue3: String,
    pub analogue4: String
}
pub type SdlKeymap = _Keymap<egui::Key>;
pub type EvdevKeymap = _Keymap<String>;


pub enum Keymap {
    // SDL/X11 input does not support second player
    Sdl(_Keymap<egui::Key>),
    Evdev(_EvdevInput),
    Both {sdl_input: _Keymap<egui::Key>,evdev_input: _EvdevInput}
}
impl Default for Keymap {
    fn default() -> Self {
        Keymap::Both { sdl_input: SdlKeymap::default(), evdev_input: _EvdevInput::default()}
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
            button4:  egui::Key::R,
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
            button8: Some("AT_TRANSLATED_SET_2_KEYBOARD_KEY_O".into())
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
            player2: EvdevKeymap::default()
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
    exe_path: PathBuf,
    // Display width,height
    window_size : (u32,u32),
    fullscreen: bool,
    disable_sdl: bool,
    game_region: GameRegion,
    freeplay: bool,
    input_method: Keymap,
    // if this is none,jvs will be emulated
    jvs_path: Option<PathBuf>,
    // if this is none,rideboard,driveboard or motionboard will be emulated
    serial_port1: Option<PathBuf>,
    serial_port2: Option<PathBuf>,
    sram_path: Option<PathBuf>,
    eeprom_path: Option<PathBuf>,
    gpu_vendor: GpuType,
    debug_message: bool,
    hammer_flicker_fix: bool,
    keep_aspect_ratio: bool,
    outrun_lens_glare_enable: bool,
    //if this is none,fps_limiter should be 0
    limit_fps_target: Option<bool>,
    lets_go_jungle_render_with_mesa: bool,
    primevalhunt_mode: Option<PrimevalHuntMode>,
    lindbergh_color: LindberghColor
}

impl Default for LindberghConfig {
    fn default() -> Self {
        Self {
            exe_path: PathBuf::new(),
            window_size: (640,480),
            fullscreen: false,
            freeplay: false,
            disable_sdl: false,
            input_method: Keymap::default(),
            game_region: GameRegion::JP,
            jvs_path: None,
            serial_port1: None,
            serial_port2: None,
            sram_path: None,
            eeprom_path: None,
            gpu_vendor: GpuType::AutoDetect,
            debug_message: false,
            hammer_flicker_fix: false,
            keep_aspect_ratio: false,
            outrun_lens_glare_enable: false,
            limit_fps_target: None,
            lets_go_jungle_render_with_mesa: false,
            primevalhunt_mode: None,
            lindbergh_color: LindberghColor::YELLOW
        }
    }
}
