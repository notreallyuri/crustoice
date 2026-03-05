use crate::{services::r2, state::SharedState};
use axum::{
    Router,
    routing::{get, post},
};

pub fn router() -> Router<SharedState> {
    Router::new()
        .route("/confirm", post(r2::confirm_upload))
        .route("/url", get(r2::get_upload_url))
}
