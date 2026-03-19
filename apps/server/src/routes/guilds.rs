use crate::{
    services::{categories, channels, guilds},
    state::SharedState,
};
use axum::{
    Router,
    routing::{delete, post},
};

pub fn router() -> Router<SharedState> {
    Router::new()
        .route("/", post(guilds::prelude::create_guild))
        .route("/{guild_id}", delete(guilds::prelude::delete_guild))
        .route("/{guild_id}/invites", post(guilds::prelude::create_invite))
        .route(
            "/{guild_id}/members",
            delete(guilds::prelude::remove_member_from_guild),
        )
        .route("/{guild_id}/channels", post(channels::create_channel))
        .route("/{guild_id}/categories", post(categories::create_category))
}
