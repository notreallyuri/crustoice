use super::broadcast;
use crate::entities::prelude::*;
use crate::state::SharedState;
use scylla::value::CqlTimestamp;
use sea_orm::EntityTrait;
use shared::protocol::ServerMessage;
use shared::structures::prelude::{ChannelId, GuildId, Message, MessageId, UserId};
use uuid::Uuid;

pub async fn handle_chat(
    channel_id: ChannelId,
    content: String,
    state: &SharedState,
    author_id: UserId,
) -> Result<(), String> {
    let channel = Channels::find_by_id(&channel_id.0)
        .one(&state.db)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("Channel not found")?;

    let guild_id = GuildId(channel.guild_id);

    let message_id = Uuid::new_v4();
    let now = chrono::Utc::now();
    let created_at_ms = now.timestamp_millis();

    let channel_uuid =
        Uuid::parse_str(&channel_id.0).map_err(|e| format!("Invalid channel_id UUID: {}", e))?;
    let author_uuid =
        Uuid::parse_str(&author_id.0).map_err(|e| format!("Invalid author_id UUID: {}", e))?;

    state
        .scylla
        .query_unpaged(
            "INSERT INTO messages (channel_id, created_at, id, author_id, content) 
             VALUES (?, ?, ?, ?, ?)",
            (
                channel_uuid,
                CqlTimestamp(created_at_ms),
                message_id,
                author_uuid,
                content.clone(),
            ),
        )
        .await
        .map_err(|e| format!("Scylla insert failed: {}", e))?;

    let payload = ServerMessage::Message {
        message: Message {
            id: MessageId(message_id.to_string()),
            channel_id,
            author_id,
            content,
            created_at: now.to_rfc3339(),
            edited_at: None,
            thread_id: None,
        },
    };

    broadcast::to_guild(state, &guild_id, &payload).await;

    Ok(())
}
