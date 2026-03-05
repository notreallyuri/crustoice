use crate::services::jwt::verify_token;
use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};

pub struct AuthedUser(pub String);

impl<S> FromRequestParts<S> for AuthedUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .ok_or((
                StatusCode::UNAUTHORIZED,
                "Missing authorization header".to_string(),
            ))?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or((StatusCode::UNAUTHORIZED, "Invalid token format".to_string()))?;

        let user_id = verify_token(token).map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()))?;

        Ok(AuthedUser(user_id))
    }
}
