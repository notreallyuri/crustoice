use crate::entities::{channels, messages, prelude::*, users};
use crate::services::broadcast;
use crate::state::SharedState;
use futures::TryFutureExt;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, QueryFilter, QueryOrder, QuerySelect,
    Set,
};
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

    let channel = Channels::find_by_id(channel_id.0.clone)
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

pub async fn fetch_history(
    channel_id: ChannelId,
    before_message_id: Option<MessageId>,
    state: &SharedState,
    user_id: &UserId,
) -> Result<(), String> {
    let db = { state.lock().await.db.clone() };

    let mut condition = Condition::all().add(messages::Column::ChannelId.eq(channel_id.0.clone()));

    if let Some(cursor_id) = before_message_id {
        let reference_msg = Messages::find_by_id(cursor_id.0.clone())
            .one(&db)
            .await
            .map_err(|e| e.to_string())?
            .ok_or("Cursor message not found")?;

        condition = condition.add(messages::Column::CreatedAt.lt(reference_msg.created_at));
    }

    let message_models = Messages::find()
        .filter(condition)
        .order_by_desc(messages::Column::CreatedAt)
        .limit(50)
        .all(&db)
        .await
        .map_err(|e| e.to_string())?;

    let mut history: Vec<Message> = message_models
        .into_iter()
        .map(|msg| Message {
            id: MessageId(msg.id),
            channel_id: ChannelId(msg.channel_id),
            author_id: UserId(msg.author_id),
            content: msg.content,
            created_at: msg.created_at.to_string(),
            updated_at: msg.updated_at.map(|dt| dt.to_string()),
        })
        .collect();

    history.reverse();

    let payload = ServerMessage::ChannelHistory {
        channel_id,
        messages: history,
    };

    let guard = state.lock().await;
    guard.send_to_user(user_id, &payload);

    Ok(())
}
