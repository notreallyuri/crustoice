use crate::{services::auth, state::SharedState};
use axum::{Router, routing::post};

pub fn router() -> Router<SharedState> {
    Router::new()
        .route("/login", post(auth::prelude::login))
        .route("/register", post(auth::prelude::register))
}
