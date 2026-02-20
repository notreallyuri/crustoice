import { create } from "zustand";
import { AppStore } from "./types";
import { userService } from "./services/user-service";
import { guildService } from "./services/guild-service";
import { channelService } from "./services/channel-service";
import { webSocketService } from "./services/websocket-service";

export const useAppStore = create<AppStore>((...a) => ({
  currentUser: null,
  guilds: [],
  messages: {},
  userCache: {},
  activeChannelId: null,
  activeGuildId: null,

  ...userService(...a),
  ...guildService(...a),
  ...channelService(...a),
  ...webSocketService(...a)
}));
