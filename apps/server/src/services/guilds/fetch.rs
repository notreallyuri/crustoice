use crate::entities::{categories, channels, guild_members, prelude::*};
use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};
use shared::structures::prelude::*;

pub async fn get_user_guilds(
    db: &DatabaseConnection,
    user_id: &UserId,
) -> Result<Vec<Guild>, DbErr> {
    let memberships = GuildMembers::find()
        .filter(guild_members::Column::UserId.eq(user_id.0.clone()))
        .find_also_related(Guilds)
        .all(db)
        .await?;

    let mut result = vec![];

    for (_, guild_opt) in memberships {
        let Some(g) = guild_opt else { continue };

        let member_models = GuildMembers::find()
            .filter(guild_members::Column::GuildId.eq(g.id.clone()))
            .find_also_related(Users)
            .all(db)
            .await?;

        let members = member_models
            .into_iter()
            .filter_map(|(m, user_opt)| {
                let u = user_opt?;

                Some(GuildMember {
                    guild_id: GuildId(m.guild_id),
                    user_id: UserId(m.user_id),
                    nickname: m.nickname,
                    roles: vec![],
                    joined_at: m.joined_at.to_string(),
                    data: UserPublic {
                        id: UserId(u.id),
                        profile: UserProfile {
                            username: u.username.clone(),
                            display_name: u.display_name.unwrap_or(u.username),
                            avatar_url: u.avatar_url,
                            bio: u.bio,
                        },
                        presence: UserPresence {
                            status: Status::Offline,
                            preset: None,
                        },
                    },
                    identity: None,
                })
            })
            .collect();

        let category_models = Categories::find()
            .filter(categories::Column::GuildId.eq(g.id.clone()))
            .all(db)
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
            .filter(channels::Column::GuildId.eq(g.id.clone()))
            .all(db)
            .await?;

        let channels: Vec<MessageChannel> = channel_models
            .into_iter()
            .map(|c| MessageChannel {
                id: ChannelId(c.id),
                guild_id: GuildId(c.guild_id),
                category_id: c.category_id.map(CategoryId),
                name: c.name,
                position: c.position,
                history: vec![],
            })
            .collect();

        let default_channel_id = channels.first().map(|c| c.id.clone());

        result.push(Guild {
            id: GuildId(g.id),
            name: g.name,
            icon_url: g.icon_url,
            banner_url: g.banner_url,
            owner_id: UserId(g.owner_id),
            default_channel_id,
            members,
            categories,
            channels,
        });
    }

    Ok(result)
}
