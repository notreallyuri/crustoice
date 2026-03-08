use crate::structures::{CategoryId, UserId};
use serde::{Deserialize, Serialize};

// --- Auth ---

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

// --- User ---

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateProfileRequest {
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    pub new_password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUsernameRequest {
    pub current_password: String,
    pub new_username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateEmailRequest {
    pub current_password: String,
    pub new_email: String,
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
pub struct CreateInviteRequest {
    pub max_uses: i32,
    pub expires_in_seconds: Option<i64>,
    pub requires_approval: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveGuildMemberRequest {
    pub user_id: UserId,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GuildInviteRequest {
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
