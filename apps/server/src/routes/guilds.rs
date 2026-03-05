use crate::{requests_http::channels, requests_http::guilds, state::SharedState};
use axum::{
    Router,
    routing::{delete, post},
};

pub fn router() -> Router<SharedState> {
    Router::new()
        .route("/", post(guilds::create_guild))
        .route("/{guild_id}", delete(guilds::delete_guild))
        .route("/{guild_id}/invites", post(guilds::create_invite))
        .route(
            "/{guild_id}/members",
            delete(guilds::remove_member_from_guild),
        )
        .route("/{guild_id}/channels", post(channels::create_channel))
}
