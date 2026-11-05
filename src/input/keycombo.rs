use std::{fmt::Display, str::FromStr};

use bevy::{platform::collections::HashSet, prelude::*};
use serde::{Deserialize, Serialize};

use crate::input::key::Key;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Reflect, Debug, Serialize, Deserialize)]
#[serde(try_from = "&str", into = "String")]
pub struct KeyCombo {
    keys: Vec<Key>,
    extra: bool,
}

impl KeyCombo {
    pub fn is_just_pressed(&self, input: &ButtonInput<KeyCode>) -> bool {
        let mut unaccounted_pressed_keys = input.get_pressed().collect::<HashSet<_>>();
        let mut was_one_of_them_just_pressed = false;
        for key in &self.keys {
            // remove all the keys that have actually been pressed.
            // if theres a difference in size, one of these buttons was pressed.
            // otherwise, fail as one of the required keys wasnt pressed
            let prev_size = unaccounted_pressed_keys.len();
            for key in key.keys() {
                unaccounted_pressed_keys.remove(&key);
                if input.just_pressed(key) {
                    was_one_of_them_just_pressed = true;
                }
            }
            if prev_size == unaccounted_pressed_keys.len() {
                return false;
            }
        }

        // if we dont allow extra keys but extra keys are pressed, fail
        if !self.extra && !unaccounted_pressed_keys.is_empty() {
            return false;
        }

        // all keys are pressed and non extra OR we allow extra
        was_one_of_them_just_pressed
    }
}

impl FromStr for KeyCombo {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut extra = false;
        s.split('+')
            .filter_map(|x| match x.to_ascii_lowercase().replace(' ', "").as_str() {
                "extra" => {
                    extra = true;
                    None
                }
                x => Some(anyhow::Context::with_context(x.parse(), || {
                    format!("couldn't parse `{s}` as a keyboard combo")
                })),
            })
            .collect::<Result<Vec<Key>, anyhow::Error>>()
            .map(|keys| Self { extra, keys })
    }
}
impl From<&str> for KeyCombo {
    fn from(value: &str) -> Self {
        value.parse().unwrap()
    }
}

impl Display for KeyCombo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, set) in self.keys.iter().enumerate() {
            write!(f, "{set}")?;
            if i != self.keys.len() - 1 {
                write!(f, "+")?;
            }
        }
        Ok(())
    }
}
impl From<KeyCombo> for String {
    fn from(value: KeyCombo) -> Self {
        value.to_string()
    }
}
