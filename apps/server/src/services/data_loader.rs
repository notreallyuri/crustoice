use crate::entities::{channels, guild_members, guilds, prelude::*, relationships, users};
use crate::state::SharedState;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use shared::structures::ids::GuildId;
use shared::structures::{
    Guild, MessageChannel, RelationshipStatus, UserPublic, UserRelationship, ids::UserId,
};
use shared::structures::{PresenceStatus, UserPresence, UserProfile};

pub async fn load_initial_state(
    state: &SharedState,
    user_id: &UserId,
) -> Result<(Vec<Guild>, Vec<UserRelationship>), String> {
    let db = state.db.clone();

    let my_guilds: Vec<(guilds::Model, Vec<guild_members::Model>)> = Guilds::find()
        .find_with_related(GuildMembers)
        .filter(guild_members::Column::UserId.eq(user_id.0.clone()))
        .all(&db)
        .await
        .map_err(|e| e.to_string())?;

    let mut result_guilds = Vec::new();

    for (guild_model, _member) in my_guilds {
        let channel_models = Channels::find()
            .filter(channels::Column::GuildId.eq(guild_model.id.clone()))
            .all(&db)
            .await
            .unwrap_or_default();

        let channels: Vec<MessageChannel> = channel_models
            .into_iter()
            .map(|c| MessageChannel {
                id: shared::structures::ChannelId(c.id),
                guild_id: shared::structures::GuildId(c.guild_id),
                name: c.name,
                position: c.position,
                category_id: None,
                history: vec![],
            })
            .collect();

        result_guilds.push(Guild {
            id: GuildId(guild_model.id),
            owner_id: UserId(guild_model.owner_id),
            name: guild_model.name,
            banner_url: guild_model.banner_url,
            icon_url: guild_model.icon_url,
            channels,
            categories: vec![],
            members: vec![],
        });
    }

    let my_relationships: Vec<(relationships::Model, Option<users::Model>)> = Relationships::find()
        .filter(relationships::Column::UserId.eq(user_id.0.clone()))
        .find_also_related(Users)
        .all(&db)
        .await
        .map_err(|e| e.to_string())?;

    let result_relationships: Vec<UserRelationship> = my_relationships
        .into_iter()
        .filter_map(|(rel, friend_opt)| {
            let friend = friend_opt?;
            Some(UserRelationship {
                id: UserId(rel.target_id),
                user: UserPublic {
                    id: UserId(friend.id),
                    profile: UserProfile {
                        username: friend.username,
                        display_name: friend.display_name.unwrap_or_default(),
                        avatar_url: friend.avatar_url,
                        bio: friend.bio,
                    },
                    presence: UserPresence {
                        status: PresenceStatus::Offline,
                        custom_message: None,
                        activity: None,
                    },
                },
                status: match rel.status {
                    0 => RelationshipStatus::PendingOutcoming,
                    1 => RelationshipStatus::PendingIncoming,
                    2 => RelationshipStatus::Friend,
                    3 => RelationshipStatus::Blocked,
                    _ => RelationshipStatus::Friend,
                },
                since: rel.since.to_string(),
            })
        })
        .collect();

    Ok((result_guilds, result_relationships))
}
