use crate::entities::{prelude::*, users};
use axum::http::StatusCode;
use bcrypt::verify;
use sea_orm::EntityTrait;

pub async fn verify_password(
    db: &sea_orm::DatabaseConnection,
    user_id: &str,
    attempted_password: &str,
) -> Result<users::Model, (StatusCode, String)> {
    let user = Users::find_by_id(user_id.to_string())
        .one(db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "User not found".to_string()))?;

    let is_valid = verify(attempted_password, &user.password_hash).unwrap_or(false);

    if !is_valid {
        return Err((
            StatusCode::UNAUTHORIZED,
            "Incorrect current password".to_string(),
        ));
    }

    Ok(user)
}
