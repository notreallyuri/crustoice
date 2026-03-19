use super::ids::UserId;
use crate::structures::user_settings::prelude::{
    Locale, NotificationSettings, PresencePreset, UISettings,
};
use sea_orm::FromJsonQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Status {
    Online,
    Busy,
    Away,
    Invisible,
    Offline,
}

// --- 1. Wrappers ---

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: UserId,
    pub account: UserAccount,
    pub profile: UserProfile,
    pub settings: UserSettings,
    pub presence: UserPresence,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserPublic {
    pub id: UserId,
    #[serde(flatten)]
    pub profile: UserProfile,
    pub presence: UserPresence,
}

// --- 2. Components ---

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserAccount {
    pub email: String,
    pub verified: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserProfile {
    pub username: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub banner_url: Option<String>,
    pub profile_color: Option<String>,
    pub bio: Option<String>,
}

#[derive(Debug, Serialize, FromJsonQueryResult, Deserialize, Clone, PartialEq, Eq)]
pub struct UserSettings {
    pub ui: UISettings,
    pub locale: Locale,
    pub notifications: NotificationSettings,
    pub presence_presets: Vec<PresencePreset>,
    pub developer_mode: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserPresence {
    pub status: Status,
    pub preset: Option<PresencePreset>,
}
