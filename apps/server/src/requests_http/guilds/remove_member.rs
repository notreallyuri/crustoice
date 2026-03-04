pub use crate::{
    entities::{guild_members, prelude::*},
    services::jwt::verify_token,
    state::SharedState,
};
pub use axum::{
    Json,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
};
use sea_orm::{ColumnTrait, EntityTrait, ExprTrait, QueryFilter};
pub use shared::{
    requests::RemoveGuildMemberRequest,
    structures::{GuildId, UserId},
};

pub async fn remove_member_from_guild(
    State(state): State<SharedState>,
    headers: HeaderMap,
    Path(guild_id): Path<GuildId>,
    Json(payload): Json<RemoveGuildMemberRequest>,
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

    let user_id =
        crate::services::jwt::verify_token(token).map_err(|e| (StatusCode::UNAUTHORIZED, e))?;

    let db = { state.lock().await.db.clone() };

    let guild = Guilds::find_by_id(guild_id.0.clone())
        .one(&db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Guild not found".to_string()))?;

    if guild.owner_id != user_id {
        return Err((
            StatusCode::FORBIDDEN,
            "Only the guild owner can remove members".to_string(),
        ));
    }

    guild_members::Entity::delete_many()
        .filter(
            guild_members::Column::GuildId
                .eq(guild_id.0.clone())
                .and(guild_members::Column::UserId.eq(payload.user_id.0.clone())),
        )
        .exec(&db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
