use crate::{
    entities::{prelude::*, users},
    extractors::auth::AuthedUser,
    services::auth::prelude::verify_password,
    state::SharedState,
};
use axum::{Json, extract::State, http::StatusCode};
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use shared::http::requests::{
    ChangePasswordRequest, UpdateEmailRequest, UpdateProfileRequest, UpdateUsernameRequest,
};

pub async fn update_profile(
    State(state): State<SharedState>,
    AuthedUser(user_id): AuthedUser,
    Json(payload): Json<UpdateProfileRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    let user = Users::find_by_id(user_id)
        .one(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "User not found".to_string()))?;

    let mut user_am: users::ActiveModel = user.into();

    if let Some(new_display_name) = payload.display_name {
        user_am.display_name = Set(Some(new_display_name));
    }

    if let Some(new_bio) = payload.bio {
        user_am.bio = Set(Some(new_bio));
    }

    if let Some(new_avatar_url) = payload.avatar_url {
        user_am.avatar_url = Set(Some(new_avatar_url));
    }

    user_am
        .update(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn update_username(
    State(state): State<SharedState>,
    AuthedUser(user_id): AuthedUser,
    Json(payload): Json<UpdateUsernameRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    let user = verify_password(&state.db, &user_id, &payload.current_password).await?;

    let mut user_am: users::ActiveModel = user.into();
    user_am.username = Set(payload.new_username);

    user_am.update(&state.db).await.map_err(|_| {
        (
            StatusCode::CONFLICT,
            "Username is already taken or invalid".to_string(),
        )
    })?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn update_email(
    State(state): State<SharedState>,
    AuthedUser(user_id): AuthedUser,
    Json(payload): Json<UpdateEmailRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    let user = verify_password(&state.db, &user_id, &payload.current_password).await?;

    let mut user_am: users::ActiveModel = user.into();
    user_am.email = Set(payload.new_email);

    user_am
        .update(&state.db)
        .await
        .map_err(|_| (StatusCode::CONFLICT, "Email is already in use".to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn change_password(
    State(state): State<SharedState>,
    AuthedUser(user_id): AuthedUser,
    Json(payload): Json<ChangePasswordRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    let user = verify_password(&state.db, &user_id, &payload.current_password).await?;

    let new_hash = bcrypt::hash(&payload.new_password, bcrypt::DEFAULT_COST).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to hash password".to_string(),
        )
    })?;

    let mut user_am: users::ActiveModel = user.into();
    user_am.password_hash = Set(new_hash);

    user_am.update(&state.db).await.map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to update password".to_string(),
        )
    })?;

    Ok(StatusCode::NO_CONTENT)
}
