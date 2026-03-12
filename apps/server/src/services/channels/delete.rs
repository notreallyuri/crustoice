use crate::{entities::prelude::*, state::SharedState};
use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use sea_orm::EntityTrait;
use shared::structures::ids::ChannelId;

pub async fn delete_channel(
    State(state): State<SharedState>,
    Path(channel_id): Path<ChannelId>,
) -> Result<StatusCode, (StatusCode, String)> {
    let db = state.db.clone();

    let delete_res = Channels::delete_by_id(channel_id.0)
        .exec(&db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if delete_res.rows_affected == 0 {
        return Err((StatusCode::NOT_FOUND, "Channel not found".to_string()));
    }

    Ok(StatusCode::NO_CONTENT)
}
