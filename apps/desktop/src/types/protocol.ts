import { Guild } from "./guild";
import { ChatMessage } from "./intents";
import { User, UserRelationship } from "./user";

export type ServerMessage =
  | { type: "IdentityValidated"; user: User }
  | { type: "InitialState"; guilds: Guild[]; relationships: UserRelationship[] }
  | { type: "Message"; message: ChatMessage }
  | { type: "PresenceUpdate"; user: any }
  | { type: "RelationshipUpdate"; relationship: UserRelationship }
  | { type: "Error"; message: string };
