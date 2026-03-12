use crate::state::SharedState;
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use scylla::value::CqlTimestamp;
use shared::{
    http::prelude::HistoryQuery,
    structures::prelude::{ChannelId, Message, MessageId, UserId},
};
use uuid::Uuid;

pub async fn get_channel_history(
    State(state): State<SharedState>,
    Path(channel_id): Path<String>,
    Query(query): Query<HistoryQuery>,
) -> Result<Json<Vec<Message>>, (StatusCode, String)> {
    let limit = query.limit.unwrap_or(50).min(100) as i32;

    let channel_uuid = Uuid::parse_str(&channel_id).map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            format!("Invalid channel_id: {}", e),
        )
    })?;

    // 99808-1179

    let rows = if let Some(before_ms) = query.before {
        state
            .scylla
            .query_unpaged(
                "SELECT id, channel_id, author_id, content, created_at, edited_at
                 FROM messages
                 WHERE channel_id = ? AND created_at < ?
                 LIMIT ?",
                (channel_uuid, CqlTimestamp(before_ms), limit),
            )
            .await
    } else {
        state
            .scylla
            .query_unpaged(
                "SELECT id, channel_id, author_id, content, created_at, edited_at
                 FROM messages
                 WHERE channel_id = ?
                 LIMIT ?",
                (channel_uuid, limit),
            )
            .await
    }
    .map_err(|e| {
        eprintln!("Scylla query error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    let rows = rows.into_rows_result().map_err(|e| {
        eprintln!("into_rows_result error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    let mut messages: Vec<Message> = rows
        .rows::<(Uuid, Uuid, Uuid, String, CqlTimestamp, Option<CqlTimestamp>)>()
        .map_err(|e| {
            eprintln!("rows deserialization error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?
        .filter_map(|r| {
            r.map_err(|e| eprintln!("Failed to deserialize message row: {e}"))
                .ok()
        })
        .map(
            |(id, channel_id, author_id, content, created_at_ms, edited_at_ms)| Message {
                id: MessageId(id.to_string()),
                channel_id: ChannelId(channel_id.to_string()),
                author_id: UserId(author_id.to_string()),
                content,
                created_at: chrono::DateTime::from_timestamp_millis(created_at_ms.0)
                    .map(|dt| dt.to_rfc3339())
                    .unwrap_or_default(),
                edited_at: edited_at_ms.and_then(|ts| {
                    chrono::DateTime::from_timestamp_millis(ts.0).map(|dt| dt.to_rfc3339())
                }),
            },
        )
        .collect();

    messages.reverse();

    Ok(Json(messages))
}
