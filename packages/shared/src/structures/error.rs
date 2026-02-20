use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ErrorCode {
    UserNotFound,
    Unauthorized,
    RoomNotFound,
    RateLimited,
    Custom(String),
}
