import {
  ChannelId,
  ChatMessage,
  Guild,
  GuildId,
  UserId,
  UserProfile
} from "@/types";

export interface GuildRepository {
  createGuild: (name: string) => Promise<void>;
  deleteGuild: (guildId: GuildId) => Promise<void>;
  selectGuild: (guildId: GuildId) => void;
}

export interface UserRepository {
  login: (username: string) => Promise<void>;
  sendMessage: (content: string) => Promise<void>;
  fetchUser: (userId: UserId) => Promise<void>;
}

export interface ChannelRepository {
  createChannel: (
    guildId: GuildId,
    name: string,
    categoryId?: string | null
  ) => Promise<void>;
  deleteChannel: (channelId: ChannelId) => Promise<void>;
  selectChannel: (guildId: GuildId, channelId: ChannelId) => Promise<void>;
}

export interface WebSocketRepository {
  initWebSocket: () => Promise<void>;
}

export interface AppState {
  currentUser: UserProfile | null;
  guilds: Guild[];
  messages: Record<ChannelId, ChatMessage[]>;
  userCache: Record<UserId, UserProfile>;
  activeChannelId: ChannelId | null;
  activeGuildId: GuildId | null;
}

export type AppStore = AppState &
  GuildRepository &
  UserRepository &
  ChannelRepository &
  WebSocketRepository;
