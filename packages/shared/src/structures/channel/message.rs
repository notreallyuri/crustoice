use crate::structures::ids::{ChannelId, MessageId, UserId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub id: MessageId,
    pub channel_id: ChannelId,
    pub author_id: UserId,
    pub content: String,
    pub created_at: String,
    pub edited_at: Option<String>,
    pub thread_id: Option<MessageId>,
    pub deleted: bool,
}
