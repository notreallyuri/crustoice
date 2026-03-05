import {
  ChannelId,
  ChatMessage,
  Guild,
  GuildId,
  User,
  UserId,
  UserProfile
} from "@/types";
import {
  CreateChannelPayload,
  CreateGuilldPayload,
  LoginPayload,
  RegisterPayload
} from "@/types/requests";

export interface GuildRepository {
  createGuild: (
    data: CreateGuilldPayload,
    icon_path: string | null
  ) => Promise<void>;
  deleteGuild: (guildId: GuildId) => Promise<void>;
  selectGuild: (guildId: GuildId) => void;
}

export interface AuthRepository {
  initSession: () => Promise<void>;
  updateAvatar: (file: File) => Promise<void>;
  login: (payload: LoginPayload) => Promise<void>;
  register: (
    payload: RegisterPayload,
    avatar_path: string | null
  ) => Promise<void>;
}

export interface UserRepository {
  sendMessage: (content: string) => Promise<void>;
  fetchUser: (userId: UserId) => Promise<void>;
  getMe: () => Promise<void>;
  getGuilds: () => Promise<void>;
}

export interface ChannelRepository {
  createChannel: (payload: CreateChannelPayload) => Promise<void>;
  deleteChannel: (channelId: ChannelId) => Promise<void>;
  selectChannel: (guildId: GuildId, channelId: ChannelId) => Promise<void>;
}

export interface WebSocketRepository {
  initWebSocket: () => Promise<void>;
}

export type AppRepository = GuildRepository &
  UserRepository &
  ChannelRepository &
  AuthRepository &
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
