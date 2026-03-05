use crate::{requests_http::users, state::SharedState};
use axum::{
    Router,
    routing::{get, post},
};

pub fn router() -> Router<SharedState> {
    Router::new()
        .route("/@me/guilds/join", post(users::join_guild))
        .route("/@me/guilds", get(users::get_guilds))
        .route("/@me", get(users::get_me))
}
