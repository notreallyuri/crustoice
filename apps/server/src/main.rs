use axum::{Router, http::Method, routing::get};
use dotenvy::dotenv;
use state::AppState;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

pub mod entities;
pub mod extractors;
pub mod requests_http;
pub mod routes;
pub mod services;
pub mod state;
pub mod ws;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let state = Arc::new(AppState::load().await);

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_headers(Any);

    let db_for_cron = state.db.clone();
    services::cron::start_invite_cleanup(db_for_cron);

    let app = Router::new()
        .route("/ws", get(ws::ws_handler))
        .nest("/api", routes::api_router())
        .with_state(state)
        .layer(cors);

    let addr = "127.0.0.1:3000";
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Server running on {}", addr);

    axum::serve(listener, app).await.unwrap();
}
