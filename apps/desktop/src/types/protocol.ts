import { Guild, GuildMember, Message } from "./guild";
import { GuildId } from "./ids";
import { User, UserPublic, UserRelationship } from "./user";

export type ServerMessage =
  | { type: "IdentityValidated"; user: User }
  | { type: "InitialState"; guilds: Guild[]; relationships: UserRelationship[] }
  | { type: "Message"; message: Message }
  | { type: "PresenceUpdate"; user: UserPublic }
  | { type: "GuildJoined"; guild: Guild }
  | { type: "MemberJoined"; guild_id: GuildId; member: GuildMember }
  | { type: "RelationshipUpdate"; relationship: UserRelationship }
  | { type: "Error"; message: string };
