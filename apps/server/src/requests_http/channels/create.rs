use crate::{
    entities::{channels, prelude::*},
    state::SharedState,
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter,
};
use shared::{
    requests::CreateChannelRequest,
    structures::{ChannelId, GuildId, MessageChannel},
};
use uuid::Uuid;

pub async fn create_channel(
    State(state): State<SharedState>,
    Path(guild_id): Path<GuildId>,
    Json(payload): Json<CreateChannelRequest>,
) -> impl IntoResponse {
    let db = state.db.clone();

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
