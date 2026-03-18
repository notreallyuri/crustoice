use reqwest::Client;
use shared::protocol::ClientMessage;
use std::{sync::Arc, time::Duration};
use tauri::AppHandle;
use tokio::sync::{mpsc, Mutex};

pub type WsTx = mpsc::UnboundedSender<ClientMessage>;

#[derive(Debug, Default)]
pub struct ClientStore {
    pub jwt_token: Option<String>,
    pub user_id: Option<String>,
}

pub struct ClientState {
    pub store: Arc<Mutex<ClientStore>>,
    pub ws_tx: Arc<Mutex<Option<WsTx>>>,
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
            ws_tx: Arc::new(Mutex::new(None)),
            http: http_client,
        }
    }

    pub async fn disconnect_ws(&self) {
        *self.ws_tx.lock().await = None;
    }

    pub async fn connect_ws(&self, app: AppHandle, token: String) {
        let already_connected = self.ws_tx.lock().await.is_some();
        if already_connected {
            return;
        }

        let (tx, rx) = mpsc::unbounded_channel::<ClientMessage>();
        *self.ws_tx.lock().await = Some(tx);

        tokio::spawn(crate::services::ws::start_ws_client(app, token, rx));
    }

    pub async fn ws_send(&self, message: ClientMessage) -> Result<(), String> {
        let tx = self.ws_tx.lock().await;
        match tx.as_ref() {
            Some(tx) => tx.send(message).map_err(|e| e.to_string()),
            None => Err("WebSocket not connected".to_string()),
        }
    }
}

impl Default for ClientState {
    fn default() -> Self {
        Self::new()
    }
}
