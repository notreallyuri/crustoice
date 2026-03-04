use crate::{entities::prelude::*, services::jwt::verify_token, state::SharedState};
use axum::{
    extract::{Path, State},
    http::HeaderMap,
    http::StatusCode,
};
use sea_orm::EntityTrait;
use shared::structures::GuildId;

pub async fn delete_guild(
    State(state): State<SharedState>,
    headers: HeaderMap,
    Path(guild_id): Path<GuildId>,
) -> Result<StatusCode, (StatusCode, String)> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or((
            StatusCode::UNAUTHORIZED,
            "Missing Authorization header".to_string(),
        ))?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or((StatusCode::UNAUTHORIZED, "Invalid token format".to_string()))?;

    let user_id = verify_token(token).map_err(|e| (StatusCode::UNAUTHORIZED, e))?;

    let db = { state.lock().await.db.clone() };

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
