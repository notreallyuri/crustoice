use axum::extract::ws::Message;
use deadpool_redis::{Config, Pool, Runtime};
use sea_orm::DatabaseConnection;
use shared::{protocol::ServerMessage, structures::UserId};
use std::{collections::HashMap, env, sync::Arc};
use tokio::sync::{Mutex, mpsc};

pub type Tx = mpsc::UnboundedSender<Message>;

pub struct ActiveSession {
    pub tx: Tx,
    pub user_id: UserId,
}

pub struct AppState {
    pub db: DatabaseConnection,
    pub sessions: HashMap<UserId, ActiveSession>,
    pub redis: Pool,
}

pub type SharedState = Arc<Mutex<AppState>>;

impl AppState {
    pub async fn load() -> Self {
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
        let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set in .env");

        let db = sea_orm::Database::connect(db_url)
            .await
            .expect("Failed to connect to postgres");

        let cfg = Config::from_url(redis_url);
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
