use crate::structures::ChannelId;

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
    pub banner_url: Option<String>,
    pub icon_url: Option<String>,
    pub name: String,
    pub default_channel_id: Option<ChannelId>,
    pub members: Vec<GuildMember>,
    pub categories: Vec<ChannelCategory>,
    pub channels: Vec<MessageChannel>,
}
