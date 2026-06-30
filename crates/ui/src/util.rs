use std::fmt::Display;

use serde::{Deserialize as _, de::IntoDeserializer as _};

/// Deserialize a string as an Option<T>, treating empty strings as None.
///
/// # Errors
/// Returns an error if the string is not empty and cannot be deserialized into T.
pub fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => T::deserialize(s.into_deserializer()).map(Some),
    }
}

/// Deserializes switch 'on' values into boolean
///
/// # Errors
/// Returns an error if the value is not 'on' or 'off' or an empty string.
pub fn deserialize_switch<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.to_lowercase().as_str() {
        "on" | "true" => Ok(true),
        "off" | "false" | "" => Ok(false),
        other => Err(serde::de::Error::custom(format!(
            "expected 'on'/'true' or 'off'/'false', got '{other}'"
        ))),
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(transparent)]
pub struct SwitchValue(#[serde(deserialize_with = "deserialize_switch")] pub bool);

impl std::ops::Deref for SwitchValue {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for SwitchValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
