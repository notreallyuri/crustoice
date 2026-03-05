use crate::{requests_http::channels, state::SharedState};
use axum::{
    Router,
    routing::{delete, get},
};

pub fn router() -> Router<SharedState> {
    Router::new()
        .route("/{channel_id}/history", get(channels::get_channel_history))
        .route("/{channel_id}", delete(channels::delete_channel))
}
