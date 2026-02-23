use serde::{Deserialize, Serialize};

use crate::structures::{CategoryId, UserId};

// --- User ---

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub display_name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub user_id: UserId,
    pub username: String,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateProfileRequest {
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub status: Option<String>,
}

// --- Guilds ---

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateGuildRequest {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateChannelRequest {
    pub name: String,
    pub category_id: Option<CategoryId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JoinGuildRequest {
    pub invite_code: String,
}

// --- Channel ---

#[derive(Deserialize)]
pub struct HistoryQuery {
    pub before: Option<String>,
}

// --- Relationships ---

#[derive(Debug, Serialize, Deserialize)]
pub struct SendFriendRequest {
    pub target_username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRelationshipRequest {
    pub target_id: UserId,
    pub action: RelationshipAction,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RelationshipAction {
    Accept,
    Decline,
    Block,
    Remove,
}
