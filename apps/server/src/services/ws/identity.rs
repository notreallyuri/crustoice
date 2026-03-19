use super::presence;
use crate::entities::guild_members;
use crate::entities::prelude::*;
use crate::services::{guilds::fetch::get_user_guilds, users::prelude::*};
use crate::state::{ActiveSession, SharedState, Tx};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use shared::{
    protocol::ServerMessage,
    structures::prelude::{
        GuildId, Status, User, UserAccount, UserId, UserPresence, UserProfile, UserPublic,
    },
};

pub async fn handle_identify(
    state: &SharedState,
    user_id: UserId,
    socket_id: String,
    tx: Tx,
) -> Result<(), String> {
    let user_data = {
        Users::find_by_id(&user_id.0)
            .one(&state.db)
            .await
            .map_err(|e| e.to_string())?
            .ok_or("User not found".to_string())?
    };

    state
        .sessions
        .write()
        .unwrap_or_else(|e| e.into_inner())
        .entry(user_id.clone())
        .or_default()
        .insert(
            socket_id.clone(),
            ActiveSession {
                tx,
                user_id: user_id.clone(),
            },
        );

    let initial_presence = presence::get_presence(state, &user_id)
        .await
        .unwrap_or(UserPresence {
            status: Status::Online,
            preset: None,
        });

    let initial_presence = if initial_presence.status == Status::Offline {
        UserPresence {
            status: Status::Online,
            preset: None,
        }
    } else {
        initial_presence
    };

    presence::set_presence(state, &user_id, &initial_presence).await?;

    let display_name = user_data
        .display_name
        .unwrap_or_else(|| user_data.username.clone());

    let guild_memberships = GuildMembers::find()
        .filter(guild_members::Column::UserId.eq(&user_id.0))
        .all(&state.db)
        .await
        .map_err(|e| e.to_string())?;

    for membership in &guild_memberships {
        let guild_id = GuildId(membership.guild_id.clone());
        if let Err(e) = presence::add_to_guild_presence(state, &guild_id, &user_id).await {
            eprintln!(
                "Failed to add user {} to guild presence {}: {}",
                user_id.0, guild_id.0, e
            );
        }
    }

    let user = User {
        id: user_id.clone(),
        account: UserAccount {
            email: user_data.email,
            verified: true,
        },
        profile: UserProfile {
            username: user_data.username.clone(),
            display_name: display_name.clone(),
            avatar_url: user_data.avatar_url.clone(),
            banner_url: user_data.banner_url.clone(),
            bio: user_data.bio.clone(),
            profile_color: user_data.profile_color.clone(),
        },
        settings: get_full_user_settings(&user_id.0, &state.db)
            .await
            .map_err(|e| e.to_string())?,
        presence: initial_presence.clone(),
    };

    state.send_to_user(&user_id, &ServerMessage::IdentityValidated { user });

    match (
        get_user_guilds(state, &user_id).await,
        get_user_relationships(&state.db, &user_id).await,
    ) {
        (Ok(guilds), Ok(relationships)) => {
            state.send_to_user(
                &user_id,
                &ServerMessage::InitialState {
                    guilds,
                    relationships,
                },
            );
        }
        (Err(e), _) | (_, Err(e)) => {
            eprintln!("Failed to load initial state for {}: {}", user_id.0, e);
        }
    }

    let public_user = UserPublic {
        id: user_id.clone(),
        profile: UserProfile {
            username: user_data.username,
            display_name,
            avatar_url: user_data.avatar_url,
            banner_url: user_data.banner_url,
            bio: user_data.bio,
            profile_color: user_data.profile_color,
        },
        presence: initial_presence,
    };

    let online_msg = ServerMessage::PresenceUpdate { user: public_user };

    for membership in &guild_memberships {
        let guild_id = GuildId(membership.guild_id.clone());
        super::broadcast::to_guild(state, &guild_id, &online_msg).await;
    }

    super::broadcast::to_friends(state, &user_id, &online_msg).await;

    Ok(())
}

pub async fn handle_disconnect(state: &SharedState, user_id: UserId) {
    let guild_memberships = GuildMembers::find()
        .filter(guild_members::Column::UserId.eq(&user_id.0))
        .all(&state.db)
        .await
        .unwrap_or_default();

    for membership in &guild_memberships {
        let guild_id = GuildId(membership.guild_id.clone());
        let _ = presence::remove_from_guild_presence(state, &guild_id, &user_id).await;
    }

    if let Ok(Some(profile)) = get_user_profile(&state.db, &user_id).await {
        let offline_presence = UserPresence {
            status: Status::Offline,
            preset: None,
        };

        let public_user = UserPublic {
            id: user_id.clone(),
            profile,
            presence: offline_presence,
        };

        let offline_msg = ServerMessage::PresenceUpdate { user: public_user };

        for membership in &guild_memberships {
            let guild_id = GuildId(membership.guild_id.clone());
            super::broadcast::to_guild(state, &guild_id, &offline_msg).await;
        }

        super::broadcast::to_friends(state, &user_id, &offline_msg).await;
    }
}
