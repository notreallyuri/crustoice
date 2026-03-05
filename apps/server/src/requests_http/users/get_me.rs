use crate::{
    entities::{user_settings, users},
    extractors::auth::AuthedUser,
    state::SharedState,
};
use axum::{Json, extract::State, http::StatusCode};
use sea_orm::EntityTrait;
use shared::structures::{User, UserAccount, UserId, UserPresence, UserProfile};

pub async fn get_me(
    State(state): State<SharedState>,
    AuthedUser(user_id): AuthedUser,
) -> Result<Json<User>, (StatusCode, String)> {
    let db = state.db.clone();

    let (user, settings_opt) = users::Entity::find_by_id(user_id.clone())
        .find_also_related(user_settings::Entity)
        .one(&db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "User not found".to_string()))?;

    let settings = settings_opt.ok_or((
        StatusCode::INTERNAL_SERVER_ERROR,
        "User settings are missing or corrupted".to_string(),
    ))?;

    let user = User {
        id: UserId(user_id),
        account: UserAccount {
            email: user.email,
            verified: true,
        },
        profile: UserProfile {
            username: user.username.clone(),
            display_name: match user.display_name {
                Some(name) if !name.trim().is_empty() => name,
                _ => user.username.clone(),
            },
            avatar_url: user.avatar_url,
            bio: user.bio,
        },
        settings: settings.into(),
        presence: UserPresence {
            status: shared::structures::PresenceStatus::Online,
            custom_message: None,
            activity: None,
        },
    };

    Ok(Json(user))
}
