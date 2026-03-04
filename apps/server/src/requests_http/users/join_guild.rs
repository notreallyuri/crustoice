use crate::{entities::guild_members, state::SharedState};
use axum::{
    Json,
    extract::State,
    http::{HeaderMap, StatusCode},
};
use sea_orm::{ActiveModelTrait, Set};
use shared::requests::GuildInviteRequest;

pub async fn join_guild(
    State(state): State<SharedState>,
    headers: HeaderMap,
    Json(payload): Json<GuildInviteRequest>,
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
