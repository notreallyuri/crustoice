use shared::structures::{ChannelId, Guild, GuildId, Message, UserId, UserProfile};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

#[derive(Debug, Default)]
pub struct ClientStore {
    pub current_user: Option<UserProfile>,
    pub guilds: Vec<Guild>,
    pub messages: HashMap<ChannelId, Vec<Message>>,

    pub user_cache: HashMap<UserId, UserProfile>,
    pub active_channel_id: Option<ChannelId>,
    pub active_guild_id: Option<GuildId>,
}

#[derive(Default)]
pub struct ClientState {
    pub store: Arc<Mutex<ClientStore>>,
    pub ws_sender: Arc<Mutex<Option<tokio::sync::mpsc::UnboundedSender<String>>>>,
}

pub type ClientSharedState = Arc<Mutex<ClientState>>;
