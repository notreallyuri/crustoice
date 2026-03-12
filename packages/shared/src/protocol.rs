use serde::{Deserialize, Serialize};

use crate::structures::prelude::{
    ChannelId, Guild, GuildId, GuildMember, Message, User, UserPresence, UserPublic,
    UserRelationship,
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
    GuildJoined {
        guild: Guild,
    },
    MemberJoined {
        guild_id: GuildId,
        member: GuildMember,
    },
    RelationshipUpdate {
        relationship: UserRelationship,
    },
    Error {
        message: String,
    },
}
