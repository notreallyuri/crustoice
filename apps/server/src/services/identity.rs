use crate::entities::prelude::Users;
use crate::entities::relationships;
use crate::services::{presence, user_service};
use crate::state::{ActiveSession, SharedState, Tx};
use axum::extract::ws::Message;
use sea_orm::EntityTrait;
use shared::structures::UserPresence;
use shared::{
    protocol::ServerMessage,
    structures::{Guild, PresenceStatus, UserId, UserProfile},
};
use tokio::sync::mpsc::UnboundedSender;

pub async fn handle_identify(state: &SharedState, user_id: UserId, tx: Tx) -> Result<(), String> {
    let user_data = {
        let guard = state.lock().await;
        Users::find_by_id(user_id.0.clone())
            .one(&guard.db)
            .await
            .map_err(|e| e.to_string())?
    };

    let user_record = match user_data {
        Some(u) => u,
        None => return Err("User not found".to_string()),
    };

    {
        let mut guard = state.lock().await;
        guard.sessions.insert(
            user_id.clone(),
            ActiveSession {
                tx: tx.clone(),
                user_id: user_id.clone(),
            },
        );
    }

    let initial_presence = UserPresence {
        status: PresenceStatus::Online,
        custom_message: None,
        activity: None,
    };

    presence::set_presence(state, &user_id, &initial_presence).await?;

    let welcome = ServerMessage::IdentityValidated {
        user: shared::structures::user::UserProfile {
            username: user_record.username,
            display_name: user_record.display_name,
            avatar_url: user_record.avatar_url,
            bio: user_record.bio,
        },
    };

    let guard = state.lock().await;
    guard.send_to_user(&user_id, &welcome);
    drop(guard);

    if let Ok((guilds, relationships)) =
        crate::services::data_loader::load_initial_state(state, &user_id).await
    {
        let initial_state_msg = ServerMessage::InitialState {
            guilds,
            relationships,
        };

        let guard = state.lock().await;
        guard.send_to_user(&user_id, &initial_state_msg);
    }

    Ok(())
}

pub async fn handle_disconnect(state: &SharedState, user_id: UserId) {
    let _ = presence::set_presence(
        &state,
        &user_id,
        UserPresence {
            status: PresenceStatus::Offline,
            custom_message: None,
            activity: None,
        },
    )
    .await;

    let db = { state.lock().await.db.clone() };

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

        let offline_message = ServerMessage::PresenceUpdate { user: public_user };

        // TODO: Implement broadcast service
    }
}
