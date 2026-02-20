use crate::entities::{guild_members, prelude::*};
use crate::state::SharedState;
use sea_orm::{ColumnTrait, EntityTrait, ModelTrait, QueryFilter};
use shared::protocol::ServerMessage;
use shared::structures::ids::GuildId;

pub async fn to_guild(state: &SharedState, guild_id: &GuildId, message: &ServerMessage) {
    let members = {
        let guard = state.lock().await;
        GuildMembers::find()
            .filter(guild_members::Column::GuildId.eq(guild_id.0.clone()))
            .all(&guard.db)
            .await
            .unwrap_or_default()
    };

    let guard = state.lock().await;

    for member in members {
        let user_id = shared::structures::UserId(member.user_id);

        guard.send_to_user(&user_id, message);
    }
}
