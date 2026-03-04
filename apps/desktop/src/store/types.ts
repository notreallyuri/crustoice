import {
  ChannelId,
  ChatMessage,
  Guild,
  GuildId,
  User,
  UserId,
  UserProfile
} from "@/types";

export interface GuildRepository {
  createGuild: (name: string) => Promise<void>;
  deleteGuild: (guildId: GuildId) => Promise<void>;
  selectGuild: (guildId: GuildId) => void;
}

export interface UserRepository {
  login: (email: string, password: string) => Promise<void>;
  register: (
    email: string,
    username: string,
    password: string,
    display_name?: string
  ) => Promise<void>;
  sendMessage: (content: string) => Promise<void>;
  fetchUser: (userId: UserId) => Promise<void>;
  getMe: () => Promise<void>;
  getGuilds: () => Promise<void>;
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

export type AppRepository = GuildRepository &
  UserRepository &
  ChannelRepository &
  WebSocketRepository;

export interface AppState {
  currentUser: User | null;
  guilds: Guild[];
  messages: Record<ChannelId, ChatMessage[]>;
  userCache: Record<UserId, UserProfile>;
  activeChannelId: ChannelId | null;
  activeGuildId: GuildId | null;
}

export type AppStore = AppState & AppRepository;
