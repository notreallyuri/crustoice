use crate::state::SharedState;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use shared::{
    requests::{AddMemberPayLoad, CreateGuildRequest},
    structures::{ChannelId, Guild, GuildId, MessageChannel, UserId},
};
use uuid::Uuid;

pub async fn get_guilds(
    State(state): State<SharedState>,
    Path(user_id): Path<UserId>,
) -> impl IntoResponse {
    let state = state.lock().await;

    let user_guild_ids = state.user_guilds.get(&user_id).cloned().unwrap_or_default();
    let guilds: Vec<Guild> = user_guild_ids
        .iter()
        .filter_map(|id| state.guilds.get(id).cloned())
        .collect();

    Json(guilds)
}

pub async fn create_guild(
    State(state): State<SharedState>,
    Json(payload): Json<CreateGuildRequest>,
) -> impl IntoResponse {
    let mut state = state.lock().await;

    let guild_id = GuildId(Uuid::new_v4().to_string());
    let channel_id = ChannelId(Uuid::new_v4().to_string());

    let general_channel = MessageChannel {
        id: channel_id.clone(),
        guild_id: guild_id.clone(),
        category_id: None,
        name: "general".to_string(),
        position: 0,
        history: Vec::new(),
    };

    let guild = Guild {
        id: guild_id.clone(),
        name: payload.name,
        owner_id: payload.owner_id.clone(),
        members: Vec::new(),
        categories: Vec::new(),
        channels: vec![general_channel.clone()],
    };

    state.channels.insert(channel_id, general_channel);
    state.guilds.insert(guild_id.clone(), guild.clone());

    state.link_user_to_guild(payload.owner_id, guild_id.clone());

    state.save_guilds();

    (StatusCode::CREATED, Json(guild)).into_response()
}

pub async fn delete_guild(
    State(state): State<SharedState>,
    Path(guild_id): Path<GuildId>,
) -> impl IntoResponse {
    let mut state = state.lock().await;

    if let Some(guild) = state.guilds.remove(&guild_id) {
        for channel in guild.channels {
            state.channels.remove(&channel.id);
        }

        for member in guild.members {
            if let Some(user_guild_list) = state.user_guilds.get_mut(&member.user_id) {
                user_guild_list.retain(|id| id != &guild_id);
            }
        }

        state.save_guilds();
        StatusCode::NO_CONTENT.into_response()
    } else {
        (StatusCode::NOT_FOUND, "Guild not found").into_response()
    }
}

pub async fn add_member_to_guild(
    State(state): State<SharedState>,
    Path(guild_id): Path<GuildId>,
    Json(payload): Json<AddMemberPayLoad>,
) -> impl IntoResponse {
    let mut state = state.lock().await;

    if !state.guilds.contains_key(&guild_id) {
        return (StatusCode::NOT_FOUND, "Guild not found").into_response();
    }

    state.link_user_to_guild(payload.user_id, guild_id);
    state.save_guilds();

    StatusCode::OK.into_response()
}

pub async fn remove_member_from_guild(
    State(state): State<SharedState>,
    Path((guild_id, user_id)): Path<(GuildId, UserId)>,
) -> impl IntoResponse {
    let mut state = state.lock().await;

    if let Some(guild) = state.guilds.get_mut(&guild_id) {
        guild.members.retain(|m| m.user_id != user_id);
    }

    if let Some(user_guild_list) = state.user_guilds.get_mut(&user_id) {
        user_guild_list.retain(|id| id != &guild_id);
    }

    state.save_guilds();
    StatusCode::NO_CONTENT.into_response()
}
