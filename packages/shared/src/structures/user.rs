use crate::structures::user_settings::{
    locale::Locale, notifications::NotificationSettings, ui::UISettings,
};

use super::ids::UserId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PresenceStatus {
    Online,
    Busy,
    Away,
    Invisible,
    Offline,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ActivityKind {
    Playing,
    Streaming,
    Listening,
    Reading,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RelationshipStatus {
    None,
    Friend,
    Blocked,
    PendingIncoming,
    PendingOutcoming,
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
    pub bio: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserSettings {
    pub theme: UISettings,
    pub locale: Locale,
    pub notifications: NotificationSettings,
    pub developer_mode: bool,
}

// --- 3. Activity ---

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserPresence {
    pub status: PresenceStatus,
    pub custom_message: Option<String>,
    pub activity: Option<UserActivity>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserActivity {
    pub name: String,
    pub kind: ActivityKind,
}

// --- 4. Relationship ---

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserRelationship {
    pub id: UserId,
    pub user: UserPublic,
    pub status: RelationshipStatus,
    pub since: String,
}
