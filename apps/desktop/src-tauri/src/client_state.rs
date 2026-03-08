use reqwest::Client;
use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;

#[derive(Debug, Default)]
pub struct ClientStore {
    pub jwt_token: Option<String>,
    pub user_id: Option<String>,
}

pub struct ClientState {
    pub store: Arc<Mutex<ClientStore>>,
    pub ws_sender: Arc<Mutex<Option<tokio::sync::mpsc::UnboundedSender<String>>>>,
    pub http: Client,
}

impl ClientState {
    pub fn new() -> Self {
        let http_client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap_or_default();

        Self {
            store: Arc::new(Mutex::new(ClientStore::default())),
            ws_sender: Arc::new(Mutex::new(None)),
            http: http_client,
        }
    }
}

impl Default for ClientState {
    fn default() -> Self {
        Self::new()
    }
}
