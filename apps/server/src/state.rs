use axum::extract::ws::Message;
use deadpool_redis::{Config, Pool, Runtime};
use sea_orm::DatabaseConnection;
use serde_json::from_str;
use shared::{
    protocol::ServerMessage,
    structures::{ChannelId, Guild, GuildId, GuildMember, MessageChannel, UserId, UserProfile},
};
use std::{collections::HashMap, fs, sync::Arc};
use tokio::sync::{Mutex, mpsc};

pub type Tx = mpsc::UnboundedSender<Message>;

pub struct ActiveSession {
    pub tx: Tx,
    pub user_id: UserId,
}

#[derive(Default)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub sessions: HashMap<UserId, ActiveSession>,
    pub redis: Pool,
}

pub type SharedState = Arc<Mutex<AppState>>;

impl AppState {
    pub async fn load() -> Self {
        let db_url = "postgres://user:password@127.0.0.1:5433/voice_project";
        let db = sea_orm::Database::connect(db_url)
            .await
            .expect("Failed to connect to postgres");

        let cfg = Config::from_url("redis://127.0.0.1:6379");
        let redis = cfg
            .create_pool(Some(Runtime::Tokio1))
            .expect("Failed to create redis pool");

        Self {
            db,
            sessions: HashMap::new(),
            redis,
        }
    }

    pub fn send_to_user(&self, user_id: &UserId, message: &ServerMessage) {
        if let Some(session) = self.sessions.get(user_id)
            && let Ok(json) = serde_json::to_string(message)
        {
            let _ = session.tx.send(Message::Text(json.into()));
        }
    }
}
