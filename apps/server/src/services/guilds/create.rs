use crate::entities::{channels, guilds};
use crate::extractors::auth::AuthedUser;
use crate::{entities::guild_members, state::SharedState};
use axum::{Json, extract::State, http::StatusCode};
use sea_orm::{ActiveModelTrait, Set, TransactionTrait};
use shared::structures::prelude::{Channel, ChannelMode, VoiceChannel};
use shared::{
    http::requests::prelude::CreateGuildRequest,
    structures::prelude::{ChannelId, Guild, GuildId, TextChannel, UserId},
};
use uuid::Uuid;

pub async fn create_guild(
    State(state): State<SharedState>,
    AuthedUser(user_id): AuthedUser,
    Json(payload): Json<CreateGuildRequest>,
) -> Result<(StatusCode, Json<Guild>), (StatusCode, String)> {
    let guild_id = Uuid::new_v4().to_string();
    let channel_id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().into();

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
        roles: Set(Some(serde_json::json!(["owner"]))),
        joined_at: Set(now),
        identity_enabled: Set(false),
        identity_display_name: Set(None),
        identity_avatar_url: Set(None),
        identity_bio: Set(None),
        identity_show_global_username: Set(true),
    };

    let new_text_channel = channels::ActiveModel {
        id: Set(channel_id.clone()),
        guild_id: Set(guild_id.clone()),
        name: Set("general".to_string()),
        position: Set(0),
        category_id: Set(None),
        kind: Set("text".to_string()),
        mode: Set(Some("chat".to_string())),
        bitrate: Set(None),
        user_limit: Set(None),
    };

    let new_voice_channel = channels::ActiveModel {
        id: Set(Uuid::new_v4().to_string()),
        guild_id: Set(guild_id.clone()),
        name: Set("General".to_string()),
        position: Set(1),
        category_id: Set(None),
        kind: Set("voice".to_string()),
        mode: Set(None),
        bitrate: Set(Some(64000)),
        user_limit: Set(None),
    };

    let txn = state
        .db
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

    let inserted_text_channel = new_text_channel
        .insert(&txn)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let inserted_voice_channel = new_voice_channel
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

    if let Err(e) = crate::services::ws::presence::add_to_guild_presence(
        &state,
        &GuildId(guild_id.clone()),
        &UserId(user_id.clone()),
    )
    .await
    {
        eprintln!("Failed to add owner to guild presence: {}", e);
    }

    let general_text_channel = TextChannel {
        id: ChannelId(inserted_text_channel.id),
        guild_id: GuildId(inserted_text_channel.guild_id),
        category_id: None,
        name: inserted_text_channel.name,
        position: inserted_text_channel.position,
        mode: ChannelMode::default(),
        history: Vec::new(),
        pins: Vec::new(),
    };

    let general_voice_channel = VoiceChannel {
        id: ChannelId(inserted_voice_channel.id),
        guild_id: GuildId(inserted_voice_channel.guild_id),
        category_id: None,
        name: inserted_voice_channel.name,
        position: inserted_voice_channel.position,
        bitrate: inserted_voice_channel.bitrate.unwrap_or(64000),
        user_limit: inserted_voice_channel.user_limit,
        participants: vec![],
    };

    let guild_response = Guild {
        id: GuildId(guild_id),
        name: payload.name,
        owner_id: UserId(user_id),
        icon_url: None,
        banner_url: None,
        members: Vec::new(),
        default_channel_id: Some(general_text_channel.id.clone()),
        categories: Vec::new(),
        channels: vec![
            Channel::Text(general_text_channel),
            Channel::Voice(general_voice_channel),
        ],
    };

    Ok((StatusCode::CREATED, Json(guild_response)))
}
