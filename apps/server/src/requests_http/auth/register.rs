use axum::{Json, extract::State, http::StatusCode};
use bcrypt::{DEFAULT_COST, hash};
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::entities::{prelude::*, users};
use crate::services::jwt::create_token;
use crate::state::SharedState;

use shared::{
    requests::{AuthResponse, RegisterRequest},
    structures::{
        UserId, UserSettings,
        user_settings::{
            locale::Locale,
            notifications::NotificationSettings,
            ui::{UISettings, UITheme},
        },
    },
};

pub async fn register(
    State(state): State<SharedState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    let db = state.db.clone();

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
        settings: Set(UserSettings {
            locale: Locale::EnUS,
            ui: UISettings {
                theme: UITheme::DefaultDark,
            },
            notifications: NotificationSettings { active: true },
            developer_mode: false,
        }),
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
