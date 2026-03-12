pub use crate::{
    entities::{guild_members, prelude::*},
    extractors::auth::AuthedUser,
    services::auth::jwt::verify_token,
    state::SharedState,
};
pub use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use sea_orm::{ColumnTrait, EntityTrait, ExprTrait, QueryFilter};
pub use shared::{
    http::requests::RemoveGuildMemberRequest,
    structures::ids::{GuildId, UserId},
};

pub async fn remove_member_from_guild(
    State(state): State<SharedState>,
    AuthedUser(user_id): AuthedUser,
    Path(guild_id): Path<GuildId>,
    Json(payload): Json<RemoveGuildMemberRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    let guild = Guilds::find_by_id(guild_id.0.clone())
        .one(&state.db)
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
        .exec(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}
