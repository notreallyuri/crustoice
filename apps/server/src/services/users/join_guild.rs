use crate::{
    entities::{guild_members, invites, prelude::*},
    extractors::auth::AuthedUser,
    state::SharedState,
};
use axum::{Json, extract::State, http::StatusCode};
use sea_orm::{ActiveModelTrait, EntityTrait, Set, TransactionTrait};
use shared::{
    http::requests::prelude::JoinGuildRequest,
    protocol::ServerMessage,
    structures::ids::{GuildId, UserId},
};

pub async fn join_guild(
    State(state): State<SharedState>,
    AuthedUser(user_id): AuthedUser,
    Json(payload): Json<JoinGuildRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    let txn = state
        .db
        .begin()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let invite = Invites::find_by_id(payload.invite_code)
        .one(&txn)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Invalid invite code".to_string()))?;

    let now = chrono::Utc::now().into();
    if let Some(expires_at) = invite.expires_at
        && now > expires_at
    {
        return Err((StatusCode::BAD_REQUEST, "Invite has expired".to_string()));
    }

    if invite.max_uses > 0 && invite.uses >= invite.max_uses {
        return Err((
            StatusCode::BAD_REQUEST,
            "Invite has reached its maximum uses".to_string(),
        ));
    }

    let identity_enabled = payload.identity.is_some();

    let new_member = guild_members::ActiveModel {
        guild_id: Set(invite.guild_id.clone()),
        user_id: Set(user_id.clone()),
        roles: Set(Some(serde_json::json!([]))),
        joined_at: Set(now),
        identity_enabled: Set(identity_enabled),
        identity_display_name: Set(payload.identity.as_ref().map(|i| i.display_name.clone())),
        identity_avatar_url: Set(payload.identity.as_ref().and_then(|i| i.avatar_url.clone())),
        identity_bio: Set(payload.identity.as_ref().and_then(|i| i.bio.clone())),
        identity_show_global_username: Set(payload
            .identity
            .as_ref()
            .map(|i| i.show_global_username)
            .unwrap_or(true)),
    };

    new_member.insert(&txn).await.map_err(|e| {
        let err_str = e.to_string();
        if err_str.contains("duplicate key value violates unique constraint") {
            (
                StatusCode::BAD_REQUEST,
                "You are already a member of this guild".to_string(),
            )
        } else {
            (StatusCode::INTERNAL_SERVER_ERROR, err_str)
        }
    })?;

    let mut invite_am: invites::ActiveModel = invite.clone().into();
    invite_am.uses = Set(invite.uses + 1);
    invite_am
        .update(&txn)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    txn.commit()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if let Ok(guilds) =
        crate::services::guilds::fetch::get_user_guilds(&state, &UserId(user_id.clone())).await
        && let Some(guild) = guilds.into_iter().find(|g| g.id.0 == invite.guild_id)
    {
        let new_member = guild
            .members
            .iter()
            .find(|m| m.user_id.0 == user_id)
            .cloned();

        state.send_to_user(
            &UserId(user_id.clone()),
            &ServerMessage::GuildJoined { guild },
        );

        if let Some(member) = new_member {
            let guild_id = GuildId(invite.guild_id.clone());
            let msg = ServerMessage::MemberJoined {
                guild_id: guild_id.clone(),
                member,
            };
            crate::services::ws::broadcast::to_guild(&state, &guild_id, &msg).await;
        }
    }

    Ok(StatusCode::OK)
}
