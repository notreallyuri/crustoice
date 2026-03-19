import { Channel, ChannelCategory, Guild, GuildMember, Message } from "./guild";
import { ChannelId, GuildId, MessageId } from "./ids";
import { User, UserPublic, UserRelationship } from "./user";

export type ServerMessage =
  | { type: "IdentityValidated"; user: User }
  | { type: "InitialState"; guilds: Guild[]; relationships: UserRelationship[] }
  | { type: "Message"; message: Message }
  | { type: "MessageEdited"; message: Message }
  | { type: "MessageDeleted"; channel_id: ChannelId; message_id: MessageId }
  | { type: "PresenceUpdate"; user: UserPublic }
  | { type: "ChannelCreated"; guild_id: GuildId; channel: Channel }
  | { type: "CategoryCreated"; guild_id: GuildId; category: ChannelCategory }
  | { type: "GuildJoined"; guild: Guild }
  | { type: "MemberJoined"; guild_id: GuildId; member: GuildMember }
  | { type: "RelationshipUpdate"; relationship: UserRelationship }
  | { type: "Error"; message: string };
