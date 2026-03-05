use crate::entities::{guild_members, prelude::*};
use crate::state::SharedState;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use shared::protocol::ServerMessage;
use shared::structures::ids::GuildId;

pub async fn to_guild(state: &SharedState, guild_id: &GuildId, message: &ServerMessage) {
    let members = {
        GuildMembers::find()
            .filter(guild_members::Column::GuildId.eq(guild_id.0.clone()))
            .all(&state.db)
            .await
            .unwrap_or_default()
    };

    for member in members {
        let user_id = shared::structures::UserId(member.user_id);

        state.send_to_user(&user_id, message);
    }
}
