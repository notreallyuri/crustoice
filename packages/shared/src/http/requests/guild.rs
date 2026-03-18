use crate::structures::ids::UserId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateGuildRequest {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCategoryRequest {
    pub name: String,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct JoinGuildRequest {
    pub invite_code: String,
    pub identity: Option<GuildIdentityRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildIdentityRequest {
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub show_global_username: bool,
}
