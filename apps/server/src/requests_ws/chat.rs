use crate::state::SharedState;
use shared::{
    protocol::ServerMessage,
    structures::{ChannelId, ChatMessage, MessageId, UserId},
};
use uuid::Uuid;

pub async fn chat(
    channel_id_raw: String,
    content: String,
    state: &SharedState,
    user_id_raw: String,
) {
    let channel_id = ChannelId(channel_id_raw);
    let user_id = UserId(user_id_raw);

    let mut state_guard = state.lock().await;

    let guild_id = match state_guard.channels.get(&channel_id) {
        Some(c) => c.guild_id.clone(),
        None => return, // Channel doesn't exist
    };

    let is_member = state_guard
        .user_guilds
        .get(&user_id)
        .map(|guilds| guilds.contains(&guild_id))
        .unwrap_or(false);

    if !is_member {
        println!(
            "⚠️ Blocked chat attempt: User {} is not in Guild {}",
            user_id.0, guild_id.0
        );
        return;
    }

    if let Some(channel) = state_guard.channels.get_mut(&channel_id) {
        let msg_object = ChatMessage {
            id: MessageId(Uuid::new_v4().to_string()),
            channel_id: channel_id.clone(),
            author_id: user_id.clone(),
            content,
            timestamp: chrono::Utc::now().to_rfc3339(),
        };

        channel.history.push(msg_object.clone());

        state_guard.sync_channel_to_guild(&channel_id);
        state_guard.save_guilds();

        let out_msg = ServerMessage::Message {
            message: msg_object,
        };

        state_guard.broadcast_to_guild(&guild_id, &out_msg);
    }
}
