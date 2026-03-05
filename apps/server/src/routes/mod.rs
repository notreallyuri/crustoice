pub mod auth;
pub mod channels;
pub mod guilds;
pub mod upload;
pub mod users;

use crate::state::SharedState;
use axum::Router;

pub fn api_router() -> Router<SharedState> {
    Router::new()
        .nest("/auth", auth::router())
        .nest("/users", users::router())
        .nest("/guilds", guilds::router())
        .nest("/channels", channels::router())
        .nest("/upload", upload::router())
}
