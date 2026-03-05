use serde::{Deserialize, Serialize};

use crate::structures::UserId;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub user_id: UserId,
    pub username: String,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InviteResponse {
    pub invite_code: String,
    pub guild_id: String,
    pub expires_at: Option<String>,
}
