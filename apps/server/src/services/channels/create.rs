use crate::{
    entities::{channels, prelude::*},
    state::SharedState,
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter,
};
use shared::{
    http::prelude::CreateChannelRequest,
    structures::prelude::{CategoryId, Channel, ChannelId, GuildId, TextChannel, VoiceChannel},
};
use uuid::Uuid;

pub async fn create_channel(
    State(state): State<SharedState>,
    Path(guild_id): Path<GuildId>,
    Json(payload): Json<CreateChannelRequest>,
) -> impl IntoResponse {
    let guild_exists = Guilds::find_by_id(guild_id.0.clone())
        .one(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if guild_exists.is_none() {
        return Err((StatusCode::NOT_FOUND, "Guild not found".to_string()));
    }

    let channel_count = Channels::find()
        .filter(channels::Column::GuildId.eq(guild_id.0.clone()))
        .count(&state.db)
        .await
        .unwrap_or(0) as i32;

    let new_channel_id = Uuid::new_v4().to_string();

    let response = match payload {
        CreateChannelRequest::Text {
            name,
            category_id,
            mode,
        } => {
            let category_id_str = category_id.as_ref().map(|c| c.0.clone());

            channels::ActiveModel {
                id: Set(new_channel_id.clone()),
                guild_id: Set(guild_id.0.clone()),
                name: Set(name.clone()),
                position: Set(channel_count),
                category_id: Set(category_id_str.clone()),
                kind: Set("text".to_string()),
                mode: Set(Some(mode.as_str().to_string())),
                bitrate: Set(None),
                user_limit: Set(None),
            }
            .insert(&state.db)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            Channel::Text(TextChannel {
                id: ChannelId(new_channel_id),
                guild_id: guild_id.clone(),
                category_id: category_id_str.map(CategoryId),
                name,
                position: channel_count,
                mode,
                pins: vec![],
                history: vec![],
            })
        }

        CreateChannelRequest::Voice {
            name,
            category_id,
            bitrate,
            user_limit,
        } => {
            let category_id_str = category_id.as_ref().map(|c| c.0.clone());
            let resolved_bitrate = bitrate.unwrap_or(64_000);

            channels::ActiveModel {
                id: Set(new_channel_id.clone()),
                guild_id: Set(guild_id.0.clone()),
                name: Set(name.clone()),
                position: Set(channel_count),
                category_id: Set(category_id_str.clone()),
                kind: Set("voice".to_string()),
                mode: Set(None),
                bitrate: Set(Some(resolved_bitrate)),
                user_limit: Set(user_limit),
            }
            .insert(&state.db)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            Channel::Voice(VoiceChannel {
                id: ChannelId(new_channel_id),
                guild_id: guild_id.clone(),
                category_id: category_id_str.map(CategoryId),
                name,
                position: channel_count,
                bitrate: resolved_bitrate,
                user_limit,
                participants: vec![],
            })
        }

        _ => {
            return Err((
                StatusCode::NOT_IMPLEMENTED,
                "Channel type not yet supported".to_string(),
            ));
        }
    };

    Ok((StatusCode::CREATED, Json(response)))
}
