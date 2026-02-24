use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum UITheme {
    DefaultDark,
    DefaultLight,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UISettings {
    pub theme: UITheme,
}
