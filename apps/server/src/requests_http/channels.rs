use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use shared::{
    requests::CreateChannelRequest,
    structures::{ChannelId, GuildId, MessageChannel},
};
use uuid::Uuid;

use crate::state::SharedState;

pub async fn get_channel_history(
    State(state): State<SharedState>,
    Path(channel_id): Path<ChannelId>,
) -> impl IntoResponse {
    let state = state.lock().await;

    if let Some(channel) = state.channels.get(&channel_id) {
        Json(channel.history.clone()).into_response()
    } else {
        (StatusCode::NOT_FOUND, "Channel not found").into_response()
    }
}

pub async fn create_channel(
    State(state): State<SharedState>,
    Path(guild_id): Path<GuildId>,
    Json(payload): Json<CreateChannelRequest>,
) -> impl IntoResponse {
    let mut state = state.lock().await;

    if let Some(guild) = state.guilds.get_mut(&guild_id) {
        let channel_id = ChannelId(Uuid::new_v4().to_string());

        let new_channel = MessageChannel {
            id: channel_id.clone(),
            guild_id,
            category_id: payload.category_id,
            name: payload.name,
            position: guild.channels.len() as i32,
            history: Vec::new(),
        };

        guild.channels.push(new_channel.clone());

        state.channels.insert(channel_id, new_channel.clone());
        state.save_guilds();

        (StatusCode::CREATED, Json(new_channel)).into_response()
    } else {
        (StatusCode::NOT_FOUND, "Guild not found").into_response()
    }
}

pub async fn delete_channel(
    State(state): State<SharedState>,
    Path(channel_id): Path<ChannelId>,
) -> impl IntoResponse {
    let mut state = state.lock().await;

    if let Some(channel) = state.channels.remove(&channel_id) {
        if let Some(guild) = state.guilds.get_mut(&channel.guild_id) {
            guild.channels.retain(|c| c.id != channel_id);
        }

        state.save_guilds();
        StatusCode::NO_CONTENT.into_response()
    } else {
        (StatusCode::NOT_FOUND, "Channel not found").into_response()
    }
}
