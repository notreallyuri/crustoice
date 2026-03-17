import { CategoryId, ChannelId, GuildId, MessageId, UserId } from "./ids";
import { UserPublic } from "./user";

// --- Channel Mode ---

export type ChannelMode = "chat" | "board" | "threads";

// --- Pinned Message ---

export interface PinnedMessage {
  message_id: MessageId;
  pinned_by: UserId;
  pinned_at: string;
  label: string | null;
}

// --- Message ---

export interface Message {
  id: MessageId;
  channel_id: ChannelId;
  author_id: UserId;
  content: string;
  created_at: string;
  edited_at: string | null;
  thread_id: MessageId | null;
}

// --- Channels ---

export interface TextChannel {
  id: ChannelId;
  guild_id: GuildId;
  category_id: CategoryId | null;
  name: string;
  position: number;
  mode: ChannelMode;
  pins: PinnedMessage[];
  history: Message[];
}

export interface VoiceParticipant {
  user_id: UserId;
  muted: boolean;
  deafened: boolean;
  speaking: boolean;
}

export interface VoiceChannel {
  id: ChannelId;
  guild_id: GuildId;
  category_id: CategoryId | null;
  name: string;
  position: number;
  user_limit: number | null;
  bitrate: number;
  participants: VoiceParticipant[];
}

export type Channel =
  | ({ kind: "text" } & TextChannel)
  | ({ kind: "voice" } & VoiceChannel);

// --- Category ---

export interface ChannelCategory {
  id: CategoryId;
  guild_id: GuildId;
  name: string;
  position: number;
}

// --- Guild Identity ---

export interface GuildIdentity {
  display_name: string;
  avatar_url: string | null;
  bio: string | null;
  show_global_username: boolean;
}

// --- Guild Member ---

export interface GuildMember {
  guild_id: GuildId;
  user_id: UserId;
  roles: string[];
  joined_at: string;
  data: UserPublic;
  identity: GuildIdentity | null;
}

// --- Guild ---

export interface Guild {
  id: GuildId;
  owner_id: UserId;
  name: string;
  icon_url: string | null;
  banner_url: string | null;
  default_channel_id: ChannelId | null;
  members: GuildMember[];
  categories: ChannelCategory[];
  channels: Channel[];
}
