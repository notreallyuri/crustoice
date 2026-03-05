use crate::entities::prelude::Users;
use crate::entities::user_settings::Entity;
use crate::services::{presence, user_service};
use crate::state::{ActiveSession, SharedState, Tx};
use sea_orm::EntityTrait;
use shared::structures::{User, UserAccount, UserPresence, UserProfile};
use shared::{
    protocol::ServerMessage,
    structures::{PresenceStatus, UserId},
};

pub async fn handle_identify(state: &SharedState, user_id: UserId, tx: Tx) -> Result<(), String> {
    let (user_data, settings_opt) = {
        let db = state.db.clone();

        Users::find_by_id(user_id.0.clone())
            .find_also_related(Entity)
            .one(&db)
            .await
            .map_err(|e| e.to_string())?
            .ok_or("User not found".to_string())?
    };

    let settings_data = settings_opt.ok_or("User settings are missing or corrupted".to_string())?;

    state.sessions.write().unwrap().insert(
        user_id.clone(),
        ActiveSession {
            tx: tx.clone(),
            user_id: user_id.clone(),
        },
    );

    let initial_presence = UserPresence {
        status: PresenceStatus::Online,
        custom_message: None,
        activity: None,
    };

    presence::set_presence(state, &user_id, &initial_presence).await?;

    let user = User {
        id: user_id.clone(),

        account: UserAccount {
            email: user_data.email.clone(),
            verified: true,
        },

        profile: UserProfile {
            username: user_data.username.clone(),
            display_name: user_data.display_name.unwrap_or(user_data.username.clone()),
            avatar_url: user_data.avatar_url.clone(),
            bio: user_data.bio.clone(),
        },

        settings: settings_data.into(),
        presence: UserPresence {
            status: PresenceStatus::Online,
            custom_message: None,
            activity: None,
        },
    };

    let welcome = ServerMessage::IdentityValidated { user };

    state.send_to_user(&user_id, &welcome);

    Ok(())
}

pub async fn handle_disconnect(state: &SharedState, user_id: UserId) {
    let base_presence = UserPresence {
        status: PresenceStatus::Offline,
        custom_message: None,
        activity: None,
    };

    let _ = presence::set_presence(state, &user_id, &base_presence).await;

    let db = state.db.clone();

    if let Ok(Some(profile)) = user_service::get_user_profile(&db, &user_id).await {
        let public_user = shared::structures::UserPublic {
            id: user_id.clone(),
            profile,
            presence: UserPresence {
                status: PresenceStatus::Offline,
                custom_message: None,
                activity: None,
            },
        };

        let _offline_message = ServerMessage::PresenceUpdate { user: public_user };

        // TODO: Implement broadcast service
    }
}
