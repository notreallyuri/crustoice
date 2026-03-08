use crate::{
    entities::{guilds, users},
    extractors::auth::AuthedUser,
    state::SharedState,
};
use aws_sdk_s3::presigning::PresigningConfig;
use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait};
use serde::Deserialize;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Deserialize)]
pub struct UploadParams {
    pub ext: String,
    pub resource: String,
    pub id: String,
}

pub async fn confirm_upload(
    State(state): State<SharedState>,
    Query(params): Query<UploadParams>,
    AuthedUser(user_id): AuthedUser,
) -> Result<StatusCode, (StatusCode, String)> {
    let public_base_url = std::env::var("R2_PUBLIC_URL").expect("R2_PUBLIC_URL must be set");

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    match params.resource.as_str() {
        "avatar" => {
            let final_url = format!(
                "{}/avatars/{}/pfp.{}?v={}",
                public_base_url, params.id, params.ext, timestamp
            );

            if user_id != params.id {
                return Err((StatusCode::FORBIDDEN, "Unauthorized".into()));
            }

            let user_model = users::Entity::find_by_id(params.id.clone())
                .one(&state.db)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
                .ok_or((StatusCode::NOT_FOUND, "User not found".to_string()))?;

            let mut active_user: users::ActiveModel = user_model.into();
            active_user.avatar_url = Set(Some(final_url));
            active_user
                .update(&state.db)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        }
        "guild" => {
            let final_url = format!(
                "{}/guilds/{}/icon.{}?v={}",
                public_base_url, params.id, params.ext, timestamp
            );

            let guild_model = guilds::Entity::find_by_id(params.id.clone())
                .one(&state.db)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
                .ok_or((StatusCode::NOT_FOUND, "Guild not found".to_string()))?;

            if user_id != guild_model.owner_id {
                return Err((StatusCode::FORBIDDEN, "Unauthorized".into()));
            }

            let mut active_guild: crate::entities::guilds::ActiveModel = guild_model.into();
            active_guild.icon_url = Set(Some(final_url));
            active_guild
                .update(&state.db)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        }
        _ => return Err((StatusCode::BAD_REQUEST, "Invalid resource type".to_string())),
    }

    Ok(StatusCode::OK)
}

pub async fn get_upload_url(
    State(state): State<SharedState>,
    Query(params): Query<UploadParams>,
    AuthedUser(user_id): AuthedUser,
) -> Result<Json<String>, (StatusCode, String)> {
    let allowed_extensions = ["png", "jpg", "jpeg", "webp", "gif"];
    let ext = params.ext.to_lowercase();

    if !allowed_extensions.contains(&ext.as_str()) {
        return Err((StatusCode::BAD_REQUEST, "Invalid file type".to_string()));
    }

    let content_type = match ext.as_str() {
        "png" => "image/png",
        "gif" => "image/gif",
        "webp" => "image/webp",
        _ => "image/jpeg",
    };

    let key = match params.resource.as_str() {
        "avatar" => {
            if params.id != user_id {
                return Err((
                    StatusCode::FORBIDDEN,
                    "You can only update your own avatar".into(),
                ));
            }

            format!("avatars/{}/pfp.{}", params.id, ext)
        }
        "guild" => {
            let guild = guilds::Entity::find_by_id(params.id.clone())
                .one(&state.db)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
                .ok_or((StatusCode::NOT_FOUND, "Guild not found".into()))?;

            if guild.owner_id != user_id {
                return Err((
                    StatusCode::FORBIDDEN,
                    "Only the guild owner can update the guild icon".into(),
                ));
            }

            format!("guilds/{}/icon.{}", params.id, ext)
        }
        _ => return Err((StatusCode::BAD_REQUEST, "Invalid resource type".to_string())),
    };

    let config = PresigningConfig::builder()
        .expires_in(Duration::from_secs(300))
        .build()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let presigned_req = state
        .s3
        .client
        .put_object()
        .bucket(&state.s3.bucket)
        .key(key)
        .content_type(content_type)
        .presigned(config)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(presigned_req.uri().to_string()))
}
