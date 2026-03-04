use serde::{Deserialize, Serialize};

use crate::structures::{
    ChannelId, Guild, Message, User, UserPresence, UserPublic, UserRelationship, error::ErrorCode,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClientMessage {
    Identify {
        token: String,
    },
    JoinRoom {
        channel_id: ChannelId,
    },
    Chat {
        channel_id: ChannelId,
        content: String,
    },
    SetPresence {
        presence: UserPresence,
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
