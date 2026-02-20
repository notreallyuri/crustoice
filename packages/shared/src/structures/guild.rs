use super::{
    channel::{ChannelCategory, MessageChannel},
    ids::{GuildId, UserId},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GuildMember {
    pub guild_id: GuildId,
    pub user_id: UserId,
    pub nickname: Option<String>,
    pub roles: Vec<String>,
    pub joined_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Guild {
    pub id: GuildId,
    pub owner_id: UserId,
    pub name: String,

    pub members: Vec<GuildMember>,
    pub categories: Vec<ChannelCategory>,
    pub channels: Vec<MessageChannel>,
}
