use super::presence;
use crate::state::SharedState;
use redis::cmd;
use shared::protocol::ServerMessage;
use shared::structures::ids::{GuildId, UserId};

pub async fn to_guild(state: &SharedState, guild_id: &GuildId, message: &ServerMessage) {
    let member_ids = get_online_guild_members(state, guild_id).await;

    for user_id in member_ids {
        state.send_to_user(&UserId(user_id), message);
    }
}

pub async fn to_friends(state: &SharedState, user_id: &UserId, message: &ServerMessage) {
    let friend_ids = presence::get_friend_ids(state, user_id).await;

    for friend_id in friend_ids {
        state.send_to_user(&UserId(friend_id), message);
    }
}

pub async fn get_online_guild_members(state: &SharedState, guild_id: &GuildId) -> Vec<String> {
    let mut conn = match state.redis.get().await {
        Ok(c) => c,
        Err(_) => return vec![],
    };

    cmd("SMEMBERS")
        .arg(format!("guild:{}:members", guild_id.0))
        .query_async(&mut conn)
        .await
        .unwrap_or_default()
}
