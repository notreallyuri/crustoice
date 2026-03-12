use crate::{
    entities::{invites, prelude::*},
    extractors::auth::AuthedUser,
    state::SharedState,
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use shared::{
    http::prelude::{CreateInviteRequest, InviteResponse},
    structures::ids::GuildId,
};
use uuid::Uuid;

pub async fn create_invite(
    State(state): State<SharedState>,
    AuthedUser(user_id): AuthedUser,
    Path(guild_id): Path<GuildId>,
    Json(payload): Json<CreateInviteRequest>,
) -> Result<(StatusCode, Json<InviteResponse>), (StatusCode, String)> {
    let db = state.db.clone();

    let guild = Guilds::find_by_id(guild_id.0.clone())
        .one(&db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Guild not found".to_string()))?;

    if guild.owner_id != user_id {
        return Err((
            StatusCode::FORBIDDEN,
            "Only the guild owner can create invites".to_string(),
        ));
    }

    let short_code: String = Uuid::new_v4().simple().to_string()[..8].to_string();
    let now = chrono::Utc::now().naive_utc();

    let expires_at = payload.expires_in_seconds.map(|seconds| {
        now + chrono::Duration::try_seconds(seconds).unwrap_or(chrono::Duration::zero())
    });

    let new_invite = invites::ActiveModel {
        invite_code: Set(short_code.clone()),
        guild_id: Set(guild_id.0.clone()),
        creator_id: Set(user_id),
        max_uses: Set(payload.max_uses),
        uses: Set(0),
        requires_approval: Set(payload.requires_approval),
        expires_at: Set(expires_at),
        created_at: Set(now),
    };

    new_invite
        .insert(&db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let res = InviteResponse {
        invite_code: short_code,
        guild_id: guild_id.0,
        expires_at: expires_at.map(|dt| dt.to_string()),
    };

    Ok((StatusCode::CREATED, Json(res)))
}
