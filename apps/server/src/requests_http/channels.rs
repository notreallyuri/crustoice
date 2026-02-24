use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, EntityTrait, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect,
};
use shared::{
    requests::{CreateChannelRequest, HistoryQuery},
    structures::{ChannelId, GuildId, Message, MessageChannel, MessageId, UserId},
};
use uuid::Uuid;

use crate::{
    entities::{channels, messages, prelude::*},
    state::SharedState,
};

pub async fn get_channel_history(
    State(state): State<SharedState>,
    Path(channel_id): Path<ChannelId>,
    Query(query): Query<HistoryQuery>,
) -> Result<Json<Vec<Message>>, (StatusCode, String)> {
    let db = { state.lock().await.db.clone() };

    let mut condition = Condition::all().add(messages::Column::ChannelId.eq(channel_id.0.clone()));

    if let Some(cursor_id) = query.before {
        let reference_msg = Messages::find_by_id(cursor_id.clone())
            .one(&db)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
            .ok_or((
                StatusCode::NOT_FOUND,
                "Cursor message not found".to_string(),
            ))?;

        condition = condition.add(messages::Column::CreatedAt.lt(reference_msg.created_at));
    }

    let message_models = Messages::find()
        .filter(condition)
        .order_by_desc(messages::Column::CreatedAt)
        .limit(50)
        .all(&db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut history: Vec<Message> = message_models
        .into_iter()
        .map(|msg| Message {
            id: MessageId(msg.id),
            channel_id: ChannelId(msg.channel_id),
            author_id: UserId(msg.author_id),
            content: msg.content,
            created_at: msg.created_at.to_string(),
            updated_at: msg.updated_at,
        })
        .collect();

    history.reverse();

    Ok(Json(history))
}

pub async fn create_channel(
    State(state): State<SharedState>,
    Path(guild_id): Path<GuildId>,
    Json(payload): Json<CreateChannelRequest>,
) -> impl IntoResponse {
    let db = { state.lock().await.db.clone() };

    let guild_exists = Guilds::find_by_id(guild_id.0.clone())
        .one(&db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if guild_exists.is_none() {
        return Err((StatusCode::NOT_FOUND, "Guild not found".to_string()));
    }

    let channel_count = Channels::find()
        .filter(channels::Column::GuildId.eq(guild_id.0.clone()))
        .count(&db)
        .await
        .unwrap_or(0);

    let new_channel_id = Uuid::new_v4().to_string();
    let category_id_str = payload.category_id.as_ref().map(|c| c.0.clone());

    let new_channel = channels::ActiveModel {
        id: Set(new_channel_id.clone()),
        guild_id: Set(guild_id.0.clone()),
        name: Set(payload.name.clone()),
        position: Set(channel_count as i32),
        category_id: Set(category_id_str),
    };

    let inserted_channel = new_channel
        .insert(&db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let response = MessageChannel {
        id: ChannelId(inserted_channel.id),
        guild_id: GuildId(inserted_channel.guild_id),
        name: inserted_channel.name,
        position: inserted_channel.position,
        category_id: payload.category_id,
        history: vec![],
    };

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn delete_channel(
    State(state): State<SharedState>,
    Path(channel_id): Path<ChannelId>,
) -> Result<StatusCode, (StatusCode, String)> {
    let db = { state.lock().await.db.clone() };

    let delete_res = Channels::delete_by_id(channel_id.0)
        .exec(&db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if delete_res.rows_affected == 0 {
        return Err((StatusCode::NOT_FOUND, "Channel not found".to_string()));
    }

    Ok(StatusCode::NO_CONTENT)
}
