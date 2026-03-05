use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct UISettings {
    pub dark_mode: String,
    pub theme: String,
    pub rounding: String,
    pub spacing: String,
}
