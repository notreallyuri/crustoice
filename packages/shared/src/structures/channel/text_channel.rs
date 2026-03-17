use crate::structures::{
    channel::message::Message,
    ids::{CategoryId, ChannelId, GuildId, MessageId, UserId},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ChannelMode {
    #[default]
    Chat,
    Board,
    Threads,
}

impl ChannelMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            ChannelMode::Chat => "chat",
            ChannelMode::Board => "board",
            ChannelMode::Threads => "threads",
        }
    }

    pub fn from(s: &str) -> Self {
        match s {
            "board" => ChannelMode::Board,
            "threads" => ChannelMode::Threads,
            _ => ChannelMode::Chat,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PinnedMessage {
    pub message_id: MessageId,
    pub pinned_by: UserId,
    pub pinned_at: String,
    pub label: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TextChannel {
    pub id: ChannelId,
    pub guild_id: GuildId,
    pub category_id: Option<CategoryId>,
    pub name: String,
    pub position: i32,
    #[serde(default)]
    pub mode: ChannelMode,
    #[serde(default)]
    pub pins: Vec<PinnedMessage>,
    #[serde(default)]
    pub history: Vec<Message>,
}
