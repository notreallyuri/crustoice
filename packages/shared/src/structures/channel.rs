use super::ids::{CategoryId, ChannelId, GuildId, MessageId, UserId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub id: MessageId,
    pub channel_id: ChannelId,
    pub author_id: UserId,
    pub content: String,
    pub created_at: String,
    pub edited_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChannelCategory {
    pub id: CategoryId,
    pub guild_id: GuildId,
    pub name: String,
    pub position: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageChannel {
    pub id: ChannelId,
    pub guild_id: GuildId,
    pub category_id: Option<CategoryId>,
    pub name: String,
    pub position: i32,
    #[serde(default)]
    pub history: Vec<Message>,
}
