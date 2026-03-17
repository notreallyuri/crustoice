use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InviteResponse {
    pub invite_code: String,
    pub guild_id: String,
    pub expires_at: Option<String>,
}
