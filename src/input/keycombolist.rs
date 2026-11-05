use std::{fmt::Display, str::FromStr};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::input::KeyCombo;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Reflect, Debug, Serialize, Deserialize)]
#[serde(try_from = "&str", into = "String")]
pub struct KeyComboList(Vec<KeyCombo>);
impl KeyComboList {
    pub fn is_pressed(&self, input: &ButtonInput<KeyCode>) -> bool {
        self.0.iter().any(|x| x.is_just_pressed(input))
    }
}

impl FromStr for KeyComboList {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split(';')
            .map(|x| {
                anyhow::Context::with_context(x.parse(), || {
                    format!("couldn't parse `{x}` as a list of keyboard combos")
                })
            })
            .collect::<Result<Vec<KeyCombo>, anyhow::Error>>()
            .map(Self)
    }
}
impl From<&str> for KeyComboList {
    fn from(value: &str) -> Self {
        value.parse().unwrap()
    }
}

impl Display for KeyComboList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, set) in self.0.iter().enumerate() {
            write!(f, "{set}")?;
            if i != self.0.len() - 1 {
                write!(f, ";")?;
            }
        }
        Ok(())
    }
}
impl From<KeyComboList> for String {
    fn from(value: KeyComboList) -> Self {
        value.to_string()
    }
}
