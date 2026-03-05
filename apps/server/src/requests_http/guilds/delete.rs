use crate::{entities::prelude::*, extractors::auth::AuthedUser, state::SharedState};
use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use sea_orm::EntityTrait;
use shared::structures::GuildId;

pub async fn delete_guild(
    State(state): State<SharedState>,
    Path(guild_id): Path<GuildId>,
    AuthedUser(user_id): AuthedUser,
) -> Result<StatusCode, (StatusCode, String)> {
    let db = state.db.clone();

    let guild = Guilds::find_by_id(guild_id.0.clone())
        .one(&db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Guild not found".to_string()))?;

    if guild.owner_id != user_id {
        return Err((
            StatusCode::FORBIDDEN,
            "Only the guild owner can delete this guild".to_string(),
        ));
    }

    Guilds::delete_by_id(guild_id.0)
        .exec(&db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
