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

use super::presence;

pub async fn handle_identify(state: &SharedState, user_id: UserId, tx: Tx) -> Result<(), String> {
    let user_data = {
        Users::find_by_id(user_id.0.clone())
            .one(&state.db)
            .await
            .map_err(|e| e.to_string())?
            .ok_or("User not found".to_string())?
    };

    state.sessions.write().unwrap().insert(
        user_id.clone(),
        ActiveSession {
            tx: tx.clone(),
            user_id: user_id.clone(),
        },
    );

    let initial_presence = UserPresence {
        status: Status::Online,
        preset: None,
    };
    presence::set_presence(state, &user_id, &initial_presence).await?;

    let display_name = user_data
        .display_name
        .clone()
        .unwrap_or(user_data.username.clone());

    let guild_memberships = GuildMembers::find()
        .filter(guild_members::Column::UserId.eq(user_id.0.clone()))
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
            email: user_data.email.clone(),
            verified: true,
        },
        profile: UserProfile {
            username: user_data.username.clone(),
            display_name: display_name.clone(),
            avatar_url: user_data.avatar_url.clone(),
            bio: user_data.bio.clone(),
        },
        settings: get_full_user_settings(&user_id.0, &state.db)
            .await
            .map_err(|e| e.to_string())?,
        presence: initial_presence.clone(),
    };

    state.send_to_user(&user_id, &ServerMessage::IdentityValidated { user });

    match (
        get_user_guilds(&state.db, &user_id).await,
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
            bio: user_data.bio,
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
    let offline_presence = UserPresence {
        status: Status::Offline,
        preset: None,
    };

    let _ = presence::set_presence(state, &user_id, &offline_presence).await;

    let guild_memberships = GuildMembers::find()
        .filter(guild_members::Column::UserId.eq(user_id.0.clone()))
        .all(&state.db)
        .await
        .unwrap_or_default();

    for membership in &guild_memberships {
        let guild_id = GuildId(membership.guild_id.clone());
        let _ = presence::remove_from_guild_presence(state, &guild_id, &user_id).await;
    }

    if let Ok(Some(profile)) = get_user_profile(&state.db, &user_id).await {
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
