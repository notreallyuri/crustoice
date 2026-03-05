use crate::state::SharedState;
use crate::{
    entities::{
        guild_members,
        prelude::{GuildMembers, Guilds},
    },
    extractors::auth::AuthedUser,
};
use axum::{Json, extract::State, http::StatusCode};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use shared::structures::{Guild, GuildId, UserId};

pub async fn get_guilds(
    State(state): State<SharedState>,
    AuthedUser(user_id): AuthedUser,
) -> Result<Json<Vec<Guild>>, (StatusCode, String)> {
    let db = state.db.clone();

    let my_guilds = Guilds::find()
        .find_with_related(GuildMembers)
        .filter(guild_members::Column::UserId.eq(user_id.clone()))
        .all(&db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let result = my_guilds
        .into_iter()
        .map(|(g, _)| Guild {
            id: GuildId(g.id),
            owner_id: UserId(g.owner_id),
            icon_url: g.icon_url,
            banner_url: g.banner_url,
            name: g.name,
            channels: vec![],
            categories: vec![],
            members: vec![],
        })
        .collect();

    Ok(Json(result))
}
