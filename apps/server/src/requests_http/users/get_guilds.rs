use crate::entities::{
    guild_members,
    prelude::{GuildMembers, Guilds},
};
use crate::{services::jwt::verify_token, state::SharedState};
use axum::{
    Json,
    extract::State,
    http::{HeaderMap, StatusCode},
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use shared::structures::{Guild, GuildId, UserId};

pub async fn get_guilds(
    State(state): State<SharedState>,
    headers: HeaderMap,
) -> Result<Json<Vec<Guild>>, (StatusCode, String)> {
    let db = { state.lock().await.db.clone() };

    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or((
            StatusCode::UNAUTHORIZED,
            "Missing authorization header".to_string(),
        ))?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or((StatusCode::UNAUTHORIZED, "Invalid token format".to_string()))?;

    let user_id = verify_token(token).map_err(|e| (StatusCode::UNAUTHORIZED, e))?;

    let my_guilds = Guilds::find()
        .find_with_related(GuildMembers)
        .filter(guild_members::Column::UserId.eq(user_id.clone()))
        .all(&db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut result = Vec::new();

    for (guild_model, _) in my_guilds {
        result.push(Guild {
            id: GuildId(guild_model.id),
            owner_id: UserId(guild_model.owner_id),
            icon_url: guild_model.icon_url,
            banner_url: guild_model.banner_url,
            name: guild_model.name,
            channels: vec![],
            categories: vec![],
            members: vec![],
        })
    }

    Ok(Json(result))
}
