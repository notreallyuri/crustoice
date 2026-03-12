use super::jwt::create_token;
use crate::entities::{prelude::*, users};
use crate::state::SharedState;
use axum::{Json, extract::State, http::StatusCode};
use bcrypt::verify;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use shared::{
    http::prelude::{AuthResponse, LoginRequest},
    structures::ids::UserId,
};

pub async fn login(
    State(state): State<SharedState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    let user = Users::find()
        .filter(users::Column::Email.eq(payload.email))
        .one(&state.db)
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
