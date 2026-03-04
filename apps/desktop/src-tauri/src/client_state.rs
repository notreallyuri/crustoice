use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Default)]
pub struct ClientStore {
    pub jwt_token: Option<String>,
}

#[derive(Default)]
pub struct ClientState {
    pub store: Arc<Mutex<ClientStore>>,
    pub ws_sender: Arc<Mutex<Option<tokio::sync::mpsc::UnboundedSender<String>>>>,
}

pub type ClientSharedState = Arc<Mutex<ClientState>>;
