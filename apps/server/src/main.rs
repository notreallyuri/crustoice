use axum::{
    Router,
    routing::{delete, get, post},
};
use dotenvy::dotenv;
use state::AppState;
use std::sync::Arc;
use tokio::{net::TcpListener, sync::Mutex};

pub mod entities;
pub mod requests_http;
pub mod services;
pub mod state;
pub mod ws;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let state = Arc::new(Mutex::new(AppState::load().await));

    let db_for_cron = state.lock().await.db.clone();
    services::cron::start_invite_cleanup(db_for_cron);

    let user_router = Router::new()
        .route("/guilds", get(requests_http::users::get_guilds))
        .route("/guilds/join", post(requests_http::users::join_guild))
        .route("/@me", get(requests_http::users::get_me));

    let auth_router = Router::new()
        .route("/login", post(requests_http::auth::login))
        .route("/register", post(requests_http::auth::register));

    let guild_router = Router::new()
        .route("/", post(requests_http::guilds::create_guild))
        .route("/{guild_id}", delete(requests_http::guilds::delete_guild))
        .route(
            "/{guild_id}/invites",
            post(requests_http::guilds::create_invite),
        )
        .route(
            "/{guild_id}/members",
            delete(requests_http::guilds::remove_member_from_guild),
        )
        .route(
            "/{guild_id}/channels",
            post(requests_http::channels::create_channel),
        );

    let channel_router = Router::new()
        .route(
            "/{channel_id}/history",
            get(requests_http::channels::get_channel_history),
        )
        .route(
            "/{channel_id}",
            delete(requests_http::channels::delete_channel),
        );

    let app = Router::new()
        .route("/ws", get(ws::ws_handler))
        .nest("/api/user", user_router)
        .nest("/api/auth", auth_router)
        .nest("/api/guilds", guild_router)
        .nest("/api/channels", channel_router)
        .with_state(state);

    let addr = "127.0.0.1:3000";
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Server running on {}", addr);

    axum::serve(listener, app).await.unwrap();
}
