use crate::structures::prelude::{
    Channel, ChannelCategory, ChannelId, Guild, GuildId, GuildMember, Message, MessageId,
    PinnedMessage, User, UserPresence, UserPublic, UserRelationship,
};
use serde::{Deserialize, Serialize};

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
        thread_id: Option<MessageId>,
    },
    EditMessage {
        channel_id: ChannelId,
        message_id: MessageId,
        content: String,
    },
    DeleteMessage {
        channel_id: ChannelId,
        message_id: MessageId,
    },
    PinMessage {
        channel_id: ChannelId,
        message_id: MessageId,
        label: Option<String>,
    },
    UnpinMessage {
        channel_id: ChannelId,
        message_id: MessageId,
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
    ChannelCreated {
        guild_id: GuildId,
        channel: Channel,
    },
    CategoryCreated {
        guild_id: GuildId,
        category: ChannelCategory,
    },
    Message {
        message: Message,
    },
    MessageEdited {
        message: Message,
    },
    MessageDeleted {
        channel_id: ChannelId,
        message_id: MessageId,
    },
    MessagePinned {
        channel_id: ChannelId,
        pin: PinnedMessage,
    },
    MessageUnpinned {
        channel_id: ChannelId,
        message_id: MessageId,
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
