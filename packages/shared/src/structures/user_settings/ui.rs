use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DarkMode {
    System,
    Light,
    Dark,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    Default,
    Strawberry,
    Blueberry,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Rounding {
    Default,
    None,
    Full,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Spacing {
    Default,
    Compact,
    Comfortable,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct UISettings {
    pub dark_mode: DarkMode,
    pub theme: Theme,
    pub rounding: Rounding,
    pub spacing: Spacing,
}
