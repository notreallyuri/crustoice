use crate::{
    entities::users, extractors::auth::AuthedUser, services::verify_password::verify_password,
    state::SharedState,
};
use axum::{Json, extract::State, http::StatusCode};
use sea_orm::{ActiveModelTrait, Set};
use shared::http::requests::{ChangePasswordRequest, UpdateEmailRequest, UpdateUsernameRequest};

pub async fn update_username(
    State(state): State<SharedState>,
    AuthedUser(user_id): AuthedUser,
    Json(payload): Json<UpdateUsernameRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    let db = state.db.clone();

    let user = verify_password(&db, &user_id, &payload.current_password).await?;

    let mut user_am: users::ActiveModel = user.into();
    user_am.username = Set(payload.new_username);

    user_am.update(&db).await.map_err(|_| {
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
    let db = state.db.clone();

    let user = verify_password(&db, &user_id, &payload.current_password).await?;

    let mut user_am: users::ActiveModel = user.into();
    user_am.email = Set(payload.new_email);

    user_am
        .update(&db)
        .await
        .map_err(|_| (StatusCode::CONFLICT, "Email is already in use".to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn change_password(
    State(state): State<SharedState>,
    AuthedUser(user_id): AuthedUser,
    Json(payload): Json<ChangePasswordRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    let db = state.db.clone();

    let user = verify_password(&db, &user_id, &payload.current_password).await?;

    let new_hash = bcrypt::hash(&payload.new_password, bcrypt::DEFAULT_COST).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to hash password".to_string(),
        )
    })?;

    let mut user_am: users::ActiveModel = user.into();
    user_am.password_hash = Set(new_hash);

    user_am.update(&db).await.map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to update password".to_string(),
        )
    })?;

    Ok(StatusCode::NO_CONTENT)
}
