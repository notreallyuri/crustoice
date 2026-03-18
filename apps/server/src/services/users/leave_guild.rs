use crate::{
    entities::{guild_members, guilds, prelude::*},
    extractors::auth::AuthedUser,
    state::SharedState,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, ModelTrait, QueryFilter, QueryOrder, Set,
    TransactionTrait,
};

pub async fn leave_guild(
    State(state): State<SharedState>,
    AuthedUser(user_id): AuthedUser,
    Path(guild_id): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    let txn = state
        .db
        .begin()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let member = GuildMembers::find()
        .filter(guild_members::Column::GuildId.eq(guild_id.clone()))
        .filter(guild_members::Column::UserId.eq(user_id.clone()))
        .one(&txn)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let member_record = match member {
        Some(m) => m,
        None => {
            return Err((
                StatusCode::BAD_REQUEST,
                "You are not a member of this guild".to_string(),
            ));
        }
    };

    let guild = Guilds::find_by_id(guild_id.clone())
        .one(&txn)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Guild not found".to_string()))?;

    if guild.owner_id == user_id {
        let next_owner = GuildMembers::find()
            .filter(guild_members::Column::GuildId.eq(guild_id.clone()))
            .filter(guild_members::Column::UserId.ne(user_id.clone()))
            .order_by_asc(guild_members::Column::JoinedAt)
            .one(&txn)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        if let Some(new_owner) = next_owner {
            let mut guild_am: guilds::ActiveModel = guild.into();
            guild_am.owner_id = Set(new_owner.user_id.clone());

            guild_am.update(&txn).await.map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to transfer ownership: {}", e),
                )
            })?;

            member_record
                .delete(&txn)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        } else {
            guild.delete(&txn).await.map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to delete orphaned guild: {}", e),
                )
            })?;
        }
    } else {
        member_record
            .delete(&txn)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    }

    txn.commit()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let _ = crate::services::ws::presence::remove_from_guild_presence(
        &state,
        &shared::structures::ids::GuildId(guild_id.clone()),
        &shared::structures::ids::UserId(user_id.clone()),
    )
    .await;

    let guild_still_exists = Guilds::find_by_id(guild_id.clone())
        .one(&state.db)
        .await
        .unwrap_or(None)
        .is_some();

    if !guild_still_exists && let Ok(mut conn) = state.redis.get().await {
        let _: Result<(), _> = deadpool_redis::redis::cmd("DEL")
            .arg(format!("guild:{}:members", guild_id))
            .query_async(&mut conn)
            .await;
    }

    Ok(StatusCode::NO_CONTENT)
}
