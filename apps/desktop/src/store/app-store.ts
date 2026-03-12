import { create } from "zustand";
import { AppStore } from "./types";
import { createUserService } from "./services/user-service";
import { createGuildService } from "./services/guild-service";
import { createChannelService } from "./services/channel-service";
import { createWebSocketService } from "./services/ws-service";
import { createAuthService } from "./services/auth-service";

export const useAppStore = create<AppStore>()((set, get, api) => ({
  currentUser: null,
  guilds: [],
  messages: {},
  userCache: {},
  activeChannelId: null,
  activeGuildId: null,
  ws: null,

  ...createAuthService(set, get, api),
  ...createUserService(set, get, api),
  ...createGuildService(set, get, api),
  ...createChannelService(set, get, api),
  ...createWebSocketService(set, get, api)
}));
