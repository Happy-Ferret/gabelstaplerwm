use std::collections::HashMap;

use xcb::xkb;

use wm::client::{ClientSet, ScreenSet};
use wm::config::{Mode, IGNORED_MODS_VEC};
use wm::window_system::WmCommand;

// constants for easier modifier handling
#[allow(dead_code)]
/// Symbolic constant: no modifier pressed.
pub const NO_MODIFIER: u8 = 0;
/// Symbolic constant: shift modifier pressed.
pub const SHIFT: u8 = 1;
/// Symbolic constant: control modifier pressed.
pub const CTRL: u8 = 4;
#[allow(dead_code)]
/// Symbolic constant: alt modifier pressed.
pub const ALT: u8 = 8;
#[allow(dead_code)]
/// Symbolic constant: windows/mod4 modifier pressed.
pub const MOD4: u8 = 64;
/// Symbolic constant: alt gr modifier pressed.
pub const ALTGR: u8 = 136;

/// Closure type of a callback function running on key press.
pub type KeyCallback = Box<Fn(&mut ClientSet, &mut ScreenSet) -> WmCommand>;
/// Keybinding map.
pub type Keybindings = HashMap<KeyPress, KeyCallback>;

/// a key has been pressed - keycode and modifier information.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct KeyPress {
    /// Symbolic integer representing key.
    pub code: u8,
    /// Symbolic integer representing modifier combination.
    pub mods: u8,
    /// Necessary mode for modal keybindings.
    pub mode: Mode,
}

/// Get a `KeyPress` struct from a `StateNotifyEvent`
pub fn from_key(event: &xkb::StateNotifyEvent, mode: Mode) -> KeyPress {
    let ignore_mask: u8 = !IGNORED_MODS_VEC.iter().fold(0, |a, b| a | *b as u8);
    info!("ignore_mask: {}", ignore_mask);
    KeyPress {
        code: event.xkb_type(),
        mods: event.keycode() & ignore_mask,
        mode: mode,
    }
}
