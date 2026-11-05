use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Reflect, Debug, Serialize, Deserialize,
)]
#[serde(try_from = "&str", into = "String")]
pub enum Key {
    KeyCode(KeyCode),
    Shift,
    Control,
    Alt,
    Meta,
}
impl Key {
    pub fn keys(self) -> Box<[KeyCode]> {
        match self {
            Key::KeyCode(kc) => Box::new([kc]),
            Key::Shift => Box::new([KeyCode::ShiftLeft, KeyCode::ShiftRight]),
            Key::Control => todo!(),
            Key::Alt => todo!(),
            Key::Meta => todo!(),
        }
    }
}

impl From<&str> for Key {
    fn from(value: &str) -> Self {
        value.parse().unwrap()
    }
}
impl From<Key> for String {
    fn from(value: Key) -> Self {
        value.to_string()
    }
}

macro_rules! keycode_to_key {
    ($($keycode:path = $key:literal),+ $(,)?) => {
        impl std::fmt::Display for Key {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Key::KeyCode($keycode) => write!(f, $key),)+
                    Key::Shift => write!(f, "Shift"),
                    Key::Control => write!(f, "Control"),
                    Key::Alt => write!(f, "Alt"),
                    Key::Meta => write!(f, "Meta"),
                    _ => write!(f, "{self:?}")
                }
            }
        }

        impl std::str::FromStr for Key {
            type Err = anyhow::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s.to_ascii_lowercase().replace(' ', "").as_str() {
                    $($key => Ok(Key::KeyCode($keycode)),)+
                    "shift" => Ok(Key::Shift),
                    "ctrl" | "control" => Ok(Key::Control),
                    "alt" | "opt" | "option" => Ok(Key::Alt),
                    "command" | "cmd" | "win" | "window" | "windows" | "super" | "sup" | "meta" => Ok(Key::Meta),
                    _ => anyhow::bail!("failed to parse {s} as a keycode")
                }
            }
        }
    };
}

keycode_to_key!(
    KeyCode::Backquote = "`",
    KeyCode::Backslash = "\\",
    KeyCode::BracketLeft = "[",
    KeyCode::BracketRight = "]",
    KeyCode::Comma = ",",
    KeyCode::Digit0 = "0",
    KeyCode::Digit1 = "1",
    KeyCode::Digit2 = "2",
    KeyCode::Digit3 = "3",
    KeyCode::Digit4 = "4",
    KeyCode::Digit5 = "5",
    KeyCode::Digit6 = "6",
    KeyCode::Digit7 = "7",
    KeyCode::Digit8 = "8",
    KeyCode::Digit9 = "9",
    KeyCode::Equal = "=",
    KeyCode::F1 = "f1",
    KeyCode::F2 = "f2",
    KeyCode::F3 = "f3",
    KeyCode::F4 = "f4",
    KeyCode::F5 = "f5",
    KeyCode::F6 = "f6",
    KeyCode::F7 = "f7",
    KeyCode::F8 = "f8",
    KeyCode::F9 = "f9",
    KeyCode::F10 = "f10",
    KeyCode::F11 = "f11",
    KeyCode::F12 = "f12",
    KeyCode::F13 = "f13",
    KeyCode::F14 = "f14",
    KeyCode::F15 = "f15",
    KeyCode::F16 = "f16",
    KeyCode::F17 = "f17",
    KeyCode::F18 = "f18",
    KeyCode::F19 = "f19",
    KeyCode::F20 = "f20",
    KeyCode::F21 = "f21",
    KeyCode::F22 = "f22",
    KeyCode::F23 = "f23",
    KeyCode::F24 = "f24",
    KeyCode::KeyA = "a",
    KeyCode::KeyB = "b",
    KeyCode::KeyC = "c",
    KeyCode::KeyD = "d",
    KeyCode::KeyE = "e",
    KeyCode::KeyF = "f",
    KeyCode::KeyG = "g",
    KeyCode::KeyH = "h",
    KeyCode::KeyI = "i",
    KeyCode::KeyJ = "j",
    KeyCode::KeyK = "k",
    KeyCode::KeyL = "l",
    KeyCode::KeyM = "m",
    KeyCode::KeyN = "n",
    KeyCode::KeyO = "o",
    KeyCode::KeyP = "p",
    KeyCode::KeyQ = "q",
    KeyCode::KeyR = "r",
    KeyCode::KeyS = "s",
    KeyCode::KeyT = "t",
    KeyCode::KeyU = "u",
    KeyCode::KeyV = "v",
    KeyCode::KeyW = "w",
    KeyCode::KeyX = "x",
    KeyCode::KeyY = "y",
    KeyCode::KeyZ = "z",
    KeyCode::Minus = "-",
    KeyCode::Period = ".",
    KeyCode::Quote = "'",
    KeyCode::Semicolon = ";",
    KeyCode::Slash = "/",
);
