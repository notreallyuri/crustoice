use crate::structures::ids::PresetId;
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Serialize, Deserialize, Clone, Debug)]
pub struct PresencePreset {
    pub id: PresetId,
    pub label: String,
    pub icon: PresenceIcon,
    pub timer: PresenceTimer,
    pub kind: PresenceKind,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PresenceTimer {
    Elapsed,
    Countdown { seconds: u64 },
    Off,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PresenceKind {
    Fixed,
    AppLinked { process_name: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Emoji {
    Unicode {
        value: String,
    },
    Custom {
        id: String,
        name: String,
        url: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PresenceIcon {
    CustomUpload { path_url: String },
    Emoji { emoji: Emoji },
    AppIcon { process_name: String },
}
