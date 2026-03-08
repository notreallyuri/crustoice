use std::collections::HashMap;

use crate::state::SharedState;
use crate::{
    entities::{
        channels, guild_members,
        prelude::{Channels, GuildMembers, Guilds},
    },
    extractors::auth::AuthedUser,
};
use axum::{Json, extract::State, http::StatusCode};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use shared::structures::{ChannelId, Guild, GuildId, MessageChannel, UserId};

pub async fn get_guilds(
    State(state): State<SharedState>,
    AuthedUser(user_id): AuthedUser,
) -> Result<Json<Vec<Guild>>, (StatusCode, String)> {
    let db = state.db.clone();

    let my_guilds = Guilds::find()
        .find_with_related(GuildMembers)
        .filter(guild_members::Column::UserId.eq(user_id.clone()))
        .all(&db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let guild_ids: Vec<String> = my_guilds.iter().map(|(g, _)| g.id.clone()).collect();

    let all_channels = Channels::find()
        .filter(channels::Column::GuildId.is_in(guild_ids))
        .all(&db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut channels_by_guild: HashMap<String, Vec<MessageChannel>> = HashMap::new();

    for c in all_channels {
        let channel = MessageChannel {
            id: ChannelId(c.id),
            guild_id: GuildId(c.guild_id.clone()),
            category_id: None,
            name: c.name,
            position: c.position,
            history: vec![],
        };

        channels_by_guild
            .entry(c.guild_id.clone())
            .or_default()
            .push(channel);
    }

    let result = my_guilds
        .into_iter()
        .map(|(g, _)| Guild {
            id: GuildId(g.id.clone()),
            owner_id: UserId(g.owner_id),
            icon_url: g.icon_url,
            banner_url: g.banner_url,
            default_channel_id: g.default_channel_id.map(ChannelId),
            name: g.name,

            channels: channels_by_guild.remove(&g.id).unwrap_or_default(),

            categories: vec![],
            members: vec![],
        })
        .collect();

    Ok(Json(result))
}
