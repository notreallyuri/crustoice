use crate::{
    entities::{categories, prelude::*},
    extractors::auth::AuthedUser,
    state::SharedState,
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter,
};
use shared::{
    http::prelude::CreateCategoryRequest,
    structures::prelude::{CategoryId, ChannelCategory, GuildId},
};
use uuid::Uuid;

pub async fn create_category(
    State(state): State<SharedState>,
    Path(guild_id): Path<GuildId>,
    AuthedUser(user_id): AuthedUser,
    Json(payload): Json<CreateCategoryRequest>,
) -> Result<(StatusCode, Json<ChannelCategory>), (StatusCode, String)> {
    let guild = Guilds::find_by_id(guild_id.0.clone())
        .one(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Guild not found".to_string()))?;

    if guild.owner_id != user_id {
        return Err((
            StatusCode::FORBIDDEN,
            "Only the guild owner can create categories".to_string(),
        ));
    }

    let position = Categories::find()
        .filter(categories::Column::GuildId.eq(guild_id.0.clone()))
        .count(&state.db)
        .await
        .unwrap_or(0) as i32;

    let id = Uuid::new_v4().to_string();

    categories::ActiveModel {
        id: Set(id.clone()),
        guild_id: Set(guild_id.0.clone()),
        name: Set(payload.name.clone()),
        position: Set(position),
    }
    .insert(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let category = ChannelCategory {
        id: CategoryId(id),
        guild_id: guild_id.clone(),
        name: payload.name,
        position,
    };

    Ok((StatusCode::CREATED, Json(category)))
}
