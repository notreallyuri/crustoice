use axum::{Json, extract::State, http::StatusCode};
use bcrypt::{DEFAULT_COST, hash, verify};
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::entities::{prelude::*, users};
use crate::services::jwt::create_token;
use crate::state::SharedState;
use shared::requests::{AuthResponse, LoginRequest, RegisterRequest};
use shared::structures::ids::UserId;

pub async fn register(
    State(state): State<SharedState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    let db = { state.lock().await.db.clone() };

    let existing_user = Users::find()
        .filter(
            Condition::any()
                .add(users::Column::Email.eq(payload.email.clone()))
                .add(users::Column::Username.eq(payload.username.clone())),
        )
        .one(&db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if existing_user.is_some() {
        return Err((
            StatusCode::CONFLICT,
            "Username or email already taken".to_string(),
        ));
    }

    let hashed_password = hash(payload.password, DEFAULT_COST).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to hash password".to_string(),
        )
    })?;

    let new_user_id = Uuid::new_v4().to_string();
    let new_user = users::ActiveModel {
        id: Set(new_user_id.clone()),
        display_name: Set(Some(payload.username.clone())),
        username: Set(payload.username.clone()),
        email: Set(payload.username.clone()),
        password_hash: Set(hashed_password),
        bio: Set(None),
        avatar_url: Set(None),
    };

    Users::insert(new_user)
        .exec(&db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let token = create_token(&new_user_id).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(AuthResponse {
        user_id: UserId(new_user_id),
        username: payload.username,
        token,
    }))
}

pub async fn login(
    State(state): State<SharedState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    let db = { state.lock().await.db.clone() };

    let user = Users::find()
        .filter(users::Column::Email.eq(payload.email))
        .one(&db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((
            StatusCode::UNAUTHORIZED,
            "Invalid email or password".to_string(),
        ))?;

    let is_valid = verify(payload.password, &user.password_hash).unwrap_or(false);

    if !is_valid {
        return Err((
            StatusCode::UNAUTHORIZED,
            "Invalid email or password".to_string(),
        ));
    }

    let token = create_token(&user.id).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(AuthResponse {
        user_id: UserId(user.id),
        username: user.username,
        token,
    }))
}
