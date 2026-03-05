use crate::{
    entities::{messages, prelude::*},
    state::SharedState,
};
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter, QueryOrder, QuerySelect};
use shared::{
    http::requests::HistoryQuery,
    structures::{ChannelId, Message, MessageId, UserId},
};

pub async fn get_channel_history(
    State(state): State<SharedState>,
    Path(channel_id): Path<ChannelId>,
    Query(query): Query<HistoryQuery>,
) -> Result<Json<Vec<Message>>, (StatusCode, String)> {
    let db = state.db.clone();

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
            updated_at: msg.updated_at.map(|dt| dt.to_string()).unwrap_or_default(),
        })
        .collect();

    history.reverse();

    Ok(Json(history))
}
