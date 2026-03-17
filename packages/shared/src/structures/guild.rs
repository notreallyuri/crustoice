use super::{
    channel::prelude::{Channel, ChannelCategory},
    ids::{ChannelId, GuildId, UserId},
    user::UserPublic,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GuildMember {
    pub guild_id: GuildId,
    pub user_id: UserId,
    pub roles: Vec<String>,
    pub joined_at: String,
    pub data: UserPublic,
    pub identity: Option<GuildIdentity>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GuildIdentity {
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub show_global_username: bool,
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
    pub channels: Vec<Channel>,
}
