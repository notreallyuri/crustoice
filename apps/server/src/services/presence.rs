use crate::state::SharedState;
use deadpool_redis::redis::cmd;
use shared::structures::{PresenceStatus, UserId, UserPresence};

pub async fn set_presence(
    state: &SharedState,
    user_id: &UserId,
    presence: &UserPresence,
) -> Result<(), String> {
    let pool = {
        let guard = state.lock().await;
        guard.redis.clone()
    };

    let mut conn = pool.get().await.map_err(|e| e.to_string())?;

    let json = serde_json::to_string(presence).map_err(|e| e.to_string())?;

    cmd("SETEX")
        .arg(format!("presence:{}", user_id.0))
        .arg(60)
        .arg(json)
        .query_async::<_, ()>(&mut conn)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

pub async fn get_presence(state: &SharedState, user_id: &UserId) -> Result<UserPresence, String> {
    let pool = {
        let guard = state.lock().await;
        guard.redis.clone()
    };

    let mut conn = pool.get().await.map_err(|e| e.to_string())?;

    let json = cmd("GET")
        .arg(format!("presence:{}", user_id.0))
        .query_async(&mut conn)
        .await
        .map_err(|e| e.to_string())?;

    match json {
        Some(data) => serde_json::from_str(&data).map_err(|e| e.to_string()),
        None => Ok(UserPresence {
            status: PresenceStatus::Offline,
            custom_message: None,
            activity: None,
        }),
    }
}
