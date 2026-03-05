use crate::{entities::users, extractors::auth::AuthedUser, state::SharedState};
use axum::{Json, extract::State, http::StatusCode};
use sea_orm::EntityTrait;
use shared::structures::{User, UserAccount, UserId, UserPresence, UserProfile};

pub async fn get_me(
    State(state): State<SharedState>,
    AuthedUser(user_id): AuthedUser,
) -> Result<Json<User>, (StatusCode, String)> {
    let db = state.db.clone();

    let user_model = users::Entity::find_by_id(user_id.clone())
        .one(&db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "User not found".to_string()))?;

    let user = User {
        id: UserId(user_id),
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
