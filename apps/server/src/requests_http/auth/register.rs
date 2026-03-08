use axum::{Json, extract::State, http::StatusCode};
use bcrypt::{DEFAULT_COST, hash};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, QueryFilter, Set, TransactionTrait,
};
use uuid::Uuid;

use crate::entities::{prelude::*, user_settings, users};
use crate::services::jwt::create_token;
use crate::state::SharedState;

use shared::{
    http::prelude::{AuthResponse, RegisterRequest},
    structures::UserId,
};

pub async fn register(
    State(state): State<SharedState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    let db = state.db.clone();

    let txn = db
        .begin()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let existing_user = Users::find()
        .filter(
            Condition::any()
                .add(users::Column::Email.eq(payload.email.clone()))
                .add(users::Column::Username.eq(payload.username.clone())),
        )
        .one(&txn)
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

    let display_name = match payload.display_name {
        Some(name) if !name.trim().is_empty() => Some(name),
        _ => None,
    };

    let new_user = users::ActiveModel {
        id: Set(new_user_id.clone()),
        display_name: Set(display_name),
        username: Set(payload.username.clone()),
        email: Set(payload.email.clone()),
        password_hash: Set(hashed_password),
        bio: Set(None),
        avatar_url: Set(None),
    };

    let default_settings = user_settings::ActiveModel {
        user_id: Set(new_user_id.clone()),
        locale: Set("en-US".to_string()),
        developer_mode: Set(false),
        notifications_active: Set(true),
        theme_dark_mode: Set("system".to_string()),
        theme_color: Set("default".to_string()),
        theme_rounding: Set("0.5rem".to_string()),
        theme_spacing: Set("default".to_string()),
    };

    new_user.insert(&txn).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to create user: {}", e),
        )
    })?;

    default_settings.insert(&txn).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to create settings: {}", e),
        )
    })?;

    txn.commit()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let token = create_token(&new_user_id).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(AuthResponse {
        user_id: UserId(new_user_id),
        username: payload.username,
        token,
    }))
}
