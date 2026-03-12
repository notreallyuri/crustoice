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
    let db = state.db.clone();

    let txn = db
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

    Ok(StatusCode::NO_CONTENT)
}
