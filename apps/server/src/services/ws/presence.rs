use crate::state::SharedState;
use deadpool_redis::redis::cmd;
use shared::structures::prelude::{GuildId, Status, UserId, UserPresence};

pub async fn set_presence(
    state: &SharedState,
    user_id: &UserId,
    presence: &UserPresence,
) -> Result<(), String> {
    let pool = state.redis.clone();
    let mut conn = pool.get().await.map_err(|e| e.to_string())?;

    let json = serde_json::to_string(presence).map_err(|e| e.to_string())?;

    let _: () = cmd("SETEX")
        .arg(format!("presence:{}", user_id.0))
        .arg(60)
        .arg(&json) // Pass as reference
        .query_async(&mut conn)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

pub async fn get_presence(state: &SharedState, user_id: &UserId) -> Result<UserPresence, String> {
    let pool = state.redis.clone();
    let mut conn = pool.get().await.map_err(|e| e.to_string())?;

    let json: Option<String> = cmd("GET")
        .arg(format!("presence:{}", user_id.0))
        .query_async(&mut conn)
        .await
        .map_err(|e| e.to_string())?;

    match json {
        Some(data) => serde_json::from_str(&data).map_err(|e| e.to_string()),
        None => Ok(UserPresence {
            status: Status::Offline,
            preset: None,
        }),
    }
}

pub async fn refresh_presence(state: &SharedState, user_id: &UserId) -> Result<(), String> {
    let pool = state.redis.clone();
    let mut conn = pool.get().await.map_err(|e| e.to_string())?;

    let _: () = cmd("EXPIRE")
        .arg(format!("presence:{}", user_id.0))
        .arg(60)
        .query_async(&mut conn)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

pub async fn add_to_guild_presence(
    state: &SharedState,
    guild_id: &GuildId,
    user_id: &UserId,
) -> Result<(), String> {
    let mut conn = state.redis.get().await.map_err(|e| e.to_string())?;

    let _: i64 = cmd("SADD")
        .arg(format!("guild:{}:members", guild_id.0))
        .arg(&user_id.0)
        .query_async(&mut conn)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

pub async fn remove_from_guild_presence(
    state: &SharedState,
    guild_id: &GuildId,
    user_id: &UserId,
) -> Result<(), String> {
    let mut conn = state.redis.get().await.map_err(|e| e.to_string())?;

    let _: i64 = cmd("SREM")
        .arg(format!("guild:{}:members", guild_id.0))
        .arg(&user_id.0)
        .query_async(&mut conn)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

pub async fn get_friend_ids(state: &SharedState, user_id: &UserId) -> Vec<String> {
    use crate::entities::{prelude::*, relationships};
    use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter};

    let friend_status = 1i32; // RelationshipStatus::Friend

    let relationships = Relationships::find()
        .filter(
            Condition::any()
                .add(
                    Condition::all()
                        .add(relationships::Column::UserId.eq(user_id.0.clone()))
                        .add(relationships::Column::Status.eq(friend_status)),
                )
                .add(
                    Condition::all()
                        .add(relationships::Column::TargetId.eq(user_id.0.clone()))
                        .add(relationships::Column::Status.eq(friend_status)),
                ),
        )
        .all(&state.db)
        .await
        .unwrap_or_default();

    relationships
        .into_iter()
        .map(|r| {
            if r.user_id == user_id.0 {
                r.target_id
            } else {
                r.user_id
            }
        })
        .collect()
}
