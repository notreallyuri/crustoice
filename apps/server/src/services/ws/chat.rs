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
    thread_id: Option<MessageId>,
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

    let thread_uuid = thread_id
        .as_ref()
        .map(|t| Uuid::parse_str(&t.0))
        .transpose()
        .map_err(|e| format!("Invalid thread_id: {}", e))?;

    state
    .scylla
    .query_unpaged(
        "INSERT INTO messages (channel_id, created_at, id, author_id, content, deleted, thread_id)
         VALUES (?, ?, ?, ?, ?, ?, ?)",
        (
            channel_uuid,
            CqlTimestamp(created_at_ms),
            message_id,
            author_uuid,
            content.clone(),
            false,
            thread_uuid,
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
            deleted: false,
        },
    };

    broadcast::to_guild(state, &guild_id, &payload).await;

    Ok(())
}

pub async fn handle_edit_message(
    channel_id: ChannelId,
    message_id: MessageId,
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
    let now = chrono::Utc::now();

    let msg_uuid =
        Uuid::parse_str(&message_id.0).map_err(|e| format!("Invalid message_id: {}", e))?;
    let channel_uuid =
        Uuid::parse_str(&channel_id.0).map_err(|e| format!("Invalid channel_id: {}", e))?;
    let author_uuid =
        Uuid::parse_str(&author_id.0).map_err(|e| format!("Invalid author_id: {}", e))?;

    let rows = state
        .scylla
        .query_unpaged(
            "SELECT created_at FROM messages WHERE channel_id = ? AND id = ? ALLOW FILTERING",
            (channel_uuid, msg_uuid),
        )
        .await
        .map_err(|e| e.to_string())?
        .into_rows_result()
        .map_err(|e| e.to_string())?;

    let created_at_ms: i64 = rows
        .rows::<(CqlTimestamp,)>()
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .next()
        .ok_or("Message not found")?
        .0
        .0;

    let author_check = state
        .scylla
        .query_unpaged(
            "SELECT author_id FROM messages WHERE channel_id = ? AND created_at = ? AND id = ?",
            (channel_uuid, CqlTimestamp(created_at_ms), msg_uuid),
        )
        .await
        .map_err(|e| e.to_string())?
        .into_rows_result()
        .map_err(|e| e.to_string())?;

    let stored_author: Uuid = author_check
        .rows::<(Uuid,)>()
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .next()
        .ok_or("Message not found")?
        .0;

    if stored_author != author_uuid {
        return Err("Cannot edit another user's message".to_string());
    }

    state.scylla
        .query_unpaged(
            "UPDATE messages SET content = ?, edited_at = ? WHERE channel_id = ? AND created_at = ? AND id = ?",
            (
                content.clone(),
                CqlTimestamp(now.timestamp_millis()),
                channel_uuid,
                CqlTimestamp(created_at_ms),
                msg_uuid,
            ),
        )
        .await
        .map_err(|e| format!("Scylla update failed: {}", e))?;

    let payload = ServerMessage::MessageEdited {
        message: Message {
            id: message_id,
            channel_id,
            author_id,
            content,
            created_at: chrono::DateTime::from_timestamp_millis(created_at_ms)
                .map(|dt| dt.to_rfc3339())
                .unwrap_or_default(),
            edited_at: Some(now.to_rfc3339()),
            thread_id: None,
            deleted: false,
        },
    };

    broadcast::to_guild(state, &guild_id, &payload).await;
    Ok(())
}

pub async fn handle_delete_message(
    channel_id: ChannelId,
    message_id: MessageId,
    state: &SharedState,
    author_id: UserId,
) -> Result<(), String> {
    let channel = Channels::find_by_id(&channel_id.0)
        .one(&state.db)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("Channel not found")?;

    let guild_id = GuildId(channel.guild_id);

    let msg_uuid =
        Uuid::parse_str(&message_id.0).map_err(|e| format!("Invalid message_id: {}", e))?;
    let channel_uuid =
        Uuid::parse_str(&channel_id.0).map_err(|e| format!("Invalid channel_id: {}", e))?;
    let author_uuid =
        Uuid::parse_str(&author_id.0).map_err(|e| format!("Invalid author_id: {}", e))?;

    let rows = state.scylla
        .query_unpaged(
            "SELECT created_at, author_id FROM messages WHERE channel_id = ? AND id = ? ALLOW FILTERING",
            (channel_uuid, msg_uuid),
        )
        .await
        .map_err(|e| e.to_string())?
        .into_rows_result()
        .map_err(|e| e.to_string())?;

    let (created_at_ms, stored_author) = rows
        .rows::<(CqlTimestamp, Uuid)>()
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .next()
        .ok_or("Message not found")?;

    if stored_author != author_uuid {
        return Err("Cannot delete another user's message".to_string());
    }

    state
        .scylla
        .query_unpaged(
            "UPDATE messages SET deleted = true WHERE channel_id = ? AND created_at = ? AND id = ?",
            (channel_uuid, created_at_ms, msg_uuid),
        )
        .await
        .map_err(|e| format!("Scylla update failed: {}", e))?;

    let payload = ServerMessage::MessageDeleted {
        channel_id,
        message_id,
    };

    broadcast::to_guild(state, &guild_id, &payload).await;
    Ok(())
}
