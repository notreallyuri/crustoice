import { CropResult } from "@/components/kibo-ui/image-crop";
import {
  ChannelId,
  ChatMessage,
  Guild,
  GuildId,
  User,
  UserId,
  UserPresence,
  UserPublic
} from "@/types";
import {
  CreateChannelPayload,
  CreateGuilldPayload,
  LoginPayload,
  RegisterPayload,
  UpdateEmailPayload,
  UpdatePasswordPayload,
  UpdateProfilePayload,
  UpdateUsernamePayload
} from "@/types/requests";

export interface GuildRepository {
  createGuild: (
    data: CreateGuilldPayload,
    icon_path: string | null,
    crop?: CropResult
  ) => Promise<void>;
  deleteGuild: (guildId: GuildId) => Promise<void>;
  selectGuild: (guildId: GuildId) => void;
}

export interface AuthRepository {
  initSession: () => Promise<void>;
  login: (payload: LoginPayload) => Promise<void>;
  logout: () => Promise<void>;
  register: (
    payload: RegisterPayload,
    avatar_path: string | null,
    crop?: CropResult
  ) => Promise<void>;
}

export interface UserRepository {
  sendMessage: (content: string) => Promise<void>;
  fetchUser: (userId: UserId) => Promise<void>;
  getMe: () => Promise<void>;
  getGuilds: () => Promise<void>;
  leaveGuild: (guildId: GuildId) => Promise<void>;

  // Update Methods
  updateProfile: (
    payload: UpdateProfilePayload,
    crop?: CropResult
  ) => Promise<void>;
  updateUsername: (payload: UpdateUsernamePayload) => Promise<void>;
  updateEmail: (payload: UpdateEmailPayload) => Promise<void>;
  updatePassword: (payload: UpdatePasswordPayload) => Promise<void>;
}

export interface ChannelRepository {
  createChannel: (payload: CreateChannelPayload) => Promise<void>;
  deleteChannel: (channelId: ChannelId) => Promise<void>;
  selectChannel: (guildId: GuildId, channelId: ChannelId) => Promise<void>;
}

export interface WebSocketRepository {
  initWebSocket: () => Promise<void>;
  setPresence: (presence: UserPresence) => void;
  sendMessage: (channel_id: string, content: string) => void;
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
  userCache: Record<UserId, UserPublic>;
  activeChannelId: ChannelId | null;
  activeGuildId: GuildId | null;
  ws: WebSocket | null;
}

export type AppStore = AppState & AppRepository;
