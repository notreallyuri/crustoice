use crate::{requests_http::users, state::SharedState};
use axum::{
    Router,
    routing::{delete, get, patch, post},
};

pub fn router() -> Router<SharedState> {
    Router::new()
        .route("/@me/guilds/leave/{guild_id}", delete(users::leave_guild))
        .route("/@me/guilds/join", post(users::join_guild))
        .route("/@me/guilds", get(users::get_guilds))
        .route("/@me", get(users::get_me))
        .route("/@me/profile", patch(users::update_profile))
        .route("/@me/account/username", patch(users::update_username))
        .route("/@me/account/email", patch(users::update_email))
        .route("/@me/account/password", patch(users::change_password))
}
