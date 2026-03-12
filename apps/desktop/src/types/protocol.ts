import { Guild, GuildMember } from "./guild";
import { GuildId } from "./ids";
import { ChatMessage } from "./intents";
import { User, UserPublic, UserRelationship } from "./user";

export type ServerMessage =
  | { type: "IdentityValidated"; user: User }
  | { type: "InitialState"; guilds: Guild[]; relationships: UserRelationship[] }
  | { type: "Message"; message: ChatMessage }
  | { type: "PresenceUpdate"; user: UserPublic }
  | { type: "GuildJoined"; guild: Guild }
  | { type: "MemberJoined"; guild_id: GuildId; member: GuildMember }
  | { type: "RelationshipUpdate"; relationship: UserRelationship }
  | { type: "Error"; message: string };
