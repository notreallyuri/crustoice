import type { StateCreator } from "zustand";
import type { AppStore, UserRepository } from "../types";
import { invoke } from "@tauri-apps/api/core";
import { Guild, User, UserPublic } from "@/types";

export const createUserService: StateCreator<
  AppStore,
  [],
  [],
  UserRepository
> = (set, get) => ({
  async fetchUser(userId) {
    console.log("Fetching user with ID:", userId);
  },
  async getMe() {
    const user = await invoke<User>("get_me");

    set({ currentUser: user });
  },
  async getGuilds() {
    const guilds: Guild[] = await invoke<Guild[]>("get_guilds");

    const userCache: Record<string, UserPublic> = {};
    for (const guild of guilds) {
      for (const member of guild.members) {
        userCache[member.user_id] = member.data;
      }
    }

    set((state) => ({
      guilds,
      userCache: { ...state.userCache, ...userCache }
    }));
  },
  async leaveGuild(guildId) {
    await invoke("leave_guild", { guildId });
    set({ activeGuildId: null, activeChannelId: null });
    get().getGuilds();
  },

  async updateProfile(payload, pfpCrop, bannerCrop) {
    await invoke("update_profile", { payload, pfpCrop, bannerCrop });
    await get().getMe();
  },
  async updateUsername(payload) {
    await invoke("update_username", { payload });
    await get().getMe();
  },
  async updateEmail(payload) {
    await invoke("update_email", { payload });
    await get().getMe();
  },
  async updatePassword(payload) {
    await invoke("update_password", { payload });
    await get().getMe();
  }
});
