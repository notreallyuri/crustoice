use crate::{entities::guild_members, extractors::auth::AuthedUser, state::SharedState};
use axum::{Json, extract::State, http::StatusCode};
use sea_orm::{ActiveModelTrait, Set};
use shared::requests::GuildInviteRequest;

pub async fn join_guild(
    State(state): State<SharedState>,
    AuthedUser(user_id): AuthedUser,
    Json(payload): Json<GuildInviteRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    let db = state.db.clone();

    let new_member = guild_members::ActiveModel {
        guild_id: Set(payload.invite_code),
        user_id: Set(user_id),
        nickname: Set(None),
        roles: Set(Some(serde_json::json!([]))),
        joined_at: Set(chrono::Utc::now().naive_utc()),
    };

    match new_member.insert(&db).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => {
            let err_str = e.to_string();
            if err_str.contains("duplicate key value violates unique constraint") {
                Err((
                    StatusCode::BAD_REQUEST,
                    "You are already a member of this guild".to_string(),
                ))
            } else if err_str.contains("violates foreign key constraint") {
                Err((StatusCode::NOT_FOUND, "Invalid invite code".to_string()))
            } else {
                Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
            }
        }
    }
}
