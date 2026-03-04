use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Locale {
    #[serde(rename = "pt-BR")]
    PtBr,
    #[serde(rename = "en-US")]
    EnUS,
}
