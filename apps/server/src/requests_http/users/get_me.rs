use crate::{entities::users, state::SharedState};
use axum::{
    Json,
    extract::State,
    http::{HeaderMap, StatusCode},
};
use sea_orm::EntityTrait;
use shared::structures::{User, UserAccount, UserId, UserPresence, UserProfile};

pub async fn get_me(
    State(state): State<SharedState>,
    headers: HeaderMap,
) -> Result<Json<User>, (StatusCode, String)> {
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

    let raw_user_id =
        crate::services::jwt::verify_token(token).map_err(|e| (StatusCode::UNAUTHORIZED, e))?;

    let db = { state.lock().await.db.clone() };

    let user_model = users::Entity::find_by_id(raw_user_id.clone())
        .one(&db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "User not found".to_string()))?;

    let user = User {
        id: UserId(raw_user_id),
        account: UserAccount {
            email: user_model.email,
            verified: true,
        },
        profile: UserProfile {
            username: user_model.username.clone(),
            display_name: match user_model.display_name {
                Some(name) if !name.trim().is_empty() => name,
                _ => user_model.username.clone(),
            },
            avatar_url: user_model.avatar_url,
            bio: user_model.bio,
        },
        settings: user_model.settings,
        presence: UserPresence {
            status: shared::structures::PresenceStatus::Online,
            custom_message: None,
            activity: None,
        },
    };

    Ok(Json(user))
}
