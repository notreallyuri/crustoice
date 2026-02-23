use serde::{Deserialize, Serialize};

use crate::structures::{
    ChannelId, Guild, Message, PresenceStatus, User, UserId, UserPublic, UserRelationship,
    error::ErrorCode,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClientMessage {
    Identify {
        user_id: UserId,
    },
    JoinRoom {
        channel_id: ChannelId,
    },
    Chat {
        channel_id: ChannelId,
        content: String,
    },
    SetStatus {
        status: PresenceStatus,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ServerMessage {
    IdentityValidated {
        user: User,
    },
    InitialState {
        guilds: Vec<Guild>,
        relationships: Vec<UserRelationship>,
    },
    PresenceUpdate {
        user: UserPublic,
    },
    Message {
        message: Message,
    },
    RelationshipUpdate {
        relationship: UserRelationship,
    },
    Error {
        code: ErrorCode,
        message: String,
    },
}
