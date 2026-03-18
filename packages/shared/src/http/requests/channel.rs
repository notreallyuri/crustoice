use crate::structures::{channel::prelude::ChannelMode, ids::CategoryId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum CreateChannelRequest {
    Text {
        name: String,
        category_id: Option<CategoryId>,
        mode: ChannelMode,
    },
    Voice {
        name: String,
        category_id: Option<CategoryId>,
        user_limit: Option<i32>,
        bitrate: Option<i32>,
    },
    Docs {
        name: String,
        category_id: Option<CategoryId>,
    },
    Canvas {
        name: String,
        category_id: Option<CategoryId>,
    },
}

#[derive(Deserialize)]
pub struct HistoryQuery {
    pub limit: Option<i64>,
    pub before: Option<i64>,
    pub thread_id: Option<String>,
}
