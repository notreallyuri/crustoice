use crate::{requests_http::auth, state::SharedState};
use axum::{Router, routing::post};

pub fn router() -> Router<SharedState> {
    Router::new()
        .route("/login", post(auth::login))
        .route("/register", post(auth::register))
}
