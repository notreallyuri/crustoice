use aws_sdk_s3::Client;
use axum::extract::ws::Message;
use deadpool_redis::{Config, Pool, Runtime};
use scylla::client::{session::Session, session_builder::SessionBuilder};
use sea_orm::DatabaseConnection;
use shared::{protocol::ServerMessage, structures::ids::UserId};
use std::{
    collections::HashMap,
    env,
    sync::{Arc, RwLock},
};
use tokio::sync::mpsc;

pub type Tx = mpsc::UnboundedSender<Message>;

pub struct ActiveSession {
    pub tx: Tx,
    pub user_id: UserId,
}

pub struct S3 {
    pub client: Client,
    pub bucket: String,
}

pub struct AppState {
    pub s3: S3,
    pub db: DatabaseConnection,
    pub scylla: Session,
    pub sessions: RwLock<HashMap<UserId, ActiveSession>>,
    pub redis: Pool,
}

pub type SharedState = Arc<AppState>;

impl AppState {
    pub async fn load() -> Self {
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
        let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set in .env");

        let r2_endpoint = env::var("R2_ENDPOINT_URL").expect("R2_ENDPOINT_URL must be set in .env");
        let r2_access_key_id =
            env::var("R2_ACCESS_KEY_ID").expect("R2_ACCESS_KEY_ID must be set in .env");
        let r2_secret_access_key =
            env::var("R2_SECRET_ACCESS_KEY").expect("R2_SECRET_ACCESS_KEY must be set in .env");
        let r2_bucket = env::var("R2_BUCKET").expect("R2_BUCKET must be set in .env");

        let db = sea_orm::Database::connect(db_url)
            .await
            .expect("Failed to connect to postgres");

        let cfg = Config::from_url(redis_url);
        let redis = cfg
            .create_pool(Some(Runtime::Tokio1))
            .expect("Failed to create redis pool");

        let credentials = aws_sdk_s3::config::Credentials::new(
            r2_access_key_id,
            r2_secret_access_key,
            None,
            None,
            "static",
        );

        let s3_config = aws_sdk_s3::config::Builder::new()
            .behavior_version_latest()
            .endpoint_url(r2_endpoint)
            .region(aws_sdk_s3::config::Region::new("auto"))
            .credentials_provider(credentials)
            .force_path_style(true)
            .build();

        let scylla_url = env::var("SCYLLA_URL").expect("SCYLLA_URL must be set in .env");

        let scylla = SessionBuilder::new()
            .known_node(scylla_url)
            .build()
            .await
            .expect("Failed to connect to scylla");

        scylla
            .use_keyspace("chat", false)
            .await
            .expect("Failed to select keyspace");

        Self {
            s3: S3 {
                client: Client::from_conf(s3_config),
                bucket: r2_bucket,
            },
            scylla,
            db,
            sessions: RwLock::new(HashMap::new()),
            redis,
        }
    }

    pub fn send_to_user(&self, user_id: &UserId, message: &ServerMessage) {
        let sessions = self.sessions.read().unwrap_or_else(|e| e.into_inner());

        if let Some(session) = sessions.get(user_id)
            && let Ok(json) = serde_json::to_string(message)
        {
            let _ = session.tx.send(Message::Text(json.into()));
        }
    }
}
