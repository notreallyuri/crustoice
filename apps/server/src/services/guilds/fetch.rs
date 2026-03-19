use crate::{
    entities::{categories, channels, guild_members, prelude::*},
    state::SharedState,
};
use sea_orm::{ColumnTrait, DbErr, EntityTrait, QueryFilter};
use shared::structures::prelude::*;

pub async fn get_user_guilds(state: &SharedState, user_id: &UserId) -> Result<Vec<Guild>, DbErr> {
    let memberships = GuildMembers::find()
        .filter(guild_members::Column::UserId.eq(&user_id.0))
        .find_also_related(Guilds)
        .all(&state.db)
        .await?;

    let mut result = vec![];

    for (_, guild_opt) in memberships {
        let Some(g) = guild_opt else { continue };

        let member_models = GuildMembers::find()
            .filter(guild_members::Column::GuildId.eq(&g.id))
            .find_also_related(Users)
            .all(&state.db)
            .await?;

        let mut members = Vec::new();

        for (m, user_opt) in member_models {
            let Some(u) = user_opt else { continue };

            let presence =
                crate::services::ws::presence::get_presence(state, &UserId(m.user_id.clone()))
                    .await
                    .unwrap_or(UserPresence {
                        status: Status::Offline,
                        preset: None,
                    });

            members.push(GuildMember {
                guild_id: GuildId(m.guild_id),
                user_id: UserId(m.user_id),
                roles: m
                    .roles
                    .and_then(|j| serde_json::from_value(j).ok())
                    .unwrap_or_default(),
                joined_at: m.joined_at.to_string(),
                data: UserPublic {
                    id: UserId(u.id),
                    profile: UserProfile {
                        display_name: u.display_name.unwrap_or_else(|| u.username.clone()),
                        username: u.username,
                        avatar_url: u.avatar_url,
                        banner_url: u.banner_url,
                        bio: u.bio,
                        profile_color: u.profile_color,
                    },
                    presence,
                },
                identity: if m.identity_enabled {
                    Some(GuildIdentity {
                        display_name: m.identity_display_name.unwrap_or_default(),
                        avatar_url: m.identity_avatar_url,
                        bio: m.identity_bio,
                        show_global_username: m.identity_show_global_username,
                    })
                } else {
                    None
                },
            });
        }

        let category_models = Categories::find()
            .filter(categories::Column::GuildId.eq(&g.id))
            .all(&state.db)
            .await?;

        let categories: Vec<ChannelCategory> = category_models
            .into_iter()
            .map(|c| ChannelCategory {
                id: CategoryId(c.id),
                guild_id: GuildId(c.guild_id),
                name: c.name,
                position: c.position,
            })
            .collect();

        let channel_models = Channels::find()
            .filter(channels::Column::GuildId.eq(&g.id))
            .all(&state.db)
            .await?;

        let channels: Vec<Channel> = channel_models
            .into_iter()
            .map(|c| match c.kind.as_str() {
                "voice" => Channel::Voice(VoiceChannel {
                    id: ChannelId(c.id),
                    guild_id: GuildId(c.guild_id),
                    category_id: c.category_id.map(CategoryId),
                    name: c.name,
                    position: c.position,
                    user_limit: c.user_limit,
                    bitrate: c.bitrate.unwrap_or(64_000),
                    participants: vec![],
                }),
                _ => Channel::Text(TextChannel {
                    id: ChannelId(c.id),
                    guild_id: GuildId(c.guild_id),
                    category_id: c.category_id.map(CategoryId),
                    name: c.name,
                    position: c.position,
                    mode: match c.mode.as_deref() {
                        Some("board") => ChannelMode::Board,
                        Some("threads") => ChannelMode::Threads,
                        _ => ChannelMode::Chat,
                    },
                    pins: vec![],
                    history: vec![],
                }),
            })
            .collect();

        result.push(Guild {
            id: GuildId(g.id),
            name: g.name,
            icon_url: g.icon_url,
            banner_url: g.banner_url,
            owner_id: UserId(g.owner_id),
            default_channel_id: g.default_channel_id.map(ChannelId),
            members,
            categories,
            channels,
        });
    }

    Ok(result)
}
