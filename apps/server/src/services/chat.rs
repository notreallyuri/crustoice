use crate::entities::{messages, prelude::*};
use crate::services::broadcast;
use crate::state::SharedState;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use shared::protocol::ServerMessage;
use shared::structures::{ChannelId, GuildId, Message, MessageId, UserId};
use uuid::Uuid;

pub async fn handle_chat(
    channel_id: ChannelId,
    content: String,
    state: &SharedState,
    author_id: UserId,
) -> Result<(), String> {
    let db = { state.lock().await.db.clone() };

    let channel = Channels::find_by_id(channel_id.0.clone())
        .one(&db)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("Channel not found")?;

    let guild_id = GuildId(channel.guild_id);

    let new_msg_id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().naive_utc();

    let new_message = messages::ActiveModel {
        id: Set(new_msg_id.clone()),
        channel_id: Set(channel_id.0.clone()),
        author_id: Set(author_id.0.clone()),
        content: Set(content.clone()),
        created_at: Set(now),
        updated_at: Set(None),
    };

    let inserted_msg = new_message.insert(&db).await.map_err(|e| e.to_string())?;

    let payload = ServerMessage::Message {
        message: Message {
            id: MessageId(inserted_msg.id),
            channel_id: ChannelId(inserted_msg.channel_id),
            author_id: UserId(inserted_msg.author_id),
            content: inserted_msg.content,
            created_at: inserted_msg.created_at.to_string(),
            updated_at: None,
        },
    };

    broadcast::to_guild(state, &guild_id, &payload).await;

    Ok(())
}
