use crate::entities::{channels, guilds};
use crate::extractors::auth::AuthedUser;
use crate::{entities::guild_members, state::SharedState};
use axum::{Json, extract::State, http::StatusCode};
use sea_orm::{ActiveModelTrait, Set, TransactionTrait};
use shared::{
    http::requests::CreateGuildRequest,
    structures::{ChannelId, Guild, GuildId, MessageChannel, UserId},
};
use uuid::Uuid;

pub async fn create_guild(
    State(state): State<SharedState>,
    AuthedUser(user_id): AuthedUser,
    Json(payload): Json<CreateGuildRequest>,
) -> Result<(StatusCode, Json<Guild>), (StatusCode, String)> {
    let db = state.db.clone();

    let guild_id = Uuid::new_v4().to_string();
    let channel_id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().naive_utc();

    let new_guild = guilds::ActiveModel {
        id: Set(guild_id.clone()),
        owner_id: Set(user_id.clone()),
        name: Set(payload.name.clone()),
        default_channel_id: Set(None),
        banner_url: Set(None),
        icon_url: Set(None),
    };

    let new_member = guild_members::ActiveModel {
        guild_id: Set(guild_id.clone()),
        user_id: Set(user_id.clone()),
        nickname: Set(None),
        roles: Set(Some(serde_json::json!(["owner"]))),
        joined_at: Set(now),
    };

    let new_channel = channels::ActiveModel {
        id: Set(channel_id.clone()),
        guild_id: Set(guild_id.clone()),
        name: Set("general".to_string()),
        position: Set(0),
        category_id: Set(None),
    };

    let txn = db
        .begin()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let inserted_guild = new_guild
        .insert(&txn)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    new_member
        .insert(&txn)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let inserted_channel = new_channel
        .insert(&txn)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut guild_to_update: guilds::ActiveModel = inserted_guild.into();
    guild_to_update.default_channel_id = Set(Some(channel_id.clone()));

    guild_to_update
        .update(&txn)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    txn.commit()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let general_channel = MessageChannel {
        id: ChannelId(inserted_channel.id),
        guild_id: GuildId(inserted_channel.guild_id),
        category_id: None,
        name: inserted_channel.name,
        position: inserted_channel.position,
        history: Vec::new(),
    };

    let guild_response = Guild {
        id: GuildId(guild_id),
        name: payload.name,
        owner_id: UserId(user_id),
        icon_url: None,
        banner_url: None,
        members: Vec::new(),
        default_channel_id: Some(general_channel.id.clone()),
        categories: Vec::new(),
        channels: vec![general_channel],
    };

    Ok((StatusCode::CREATED, Json(guild_response)))
}
