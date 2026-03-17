import type { StateCreator } from "zustand";
import type { AppStore, UserRepository } from "../types";
import { invoke } from "@tauri-apps/api/core";
import { Guild, User } from "@/types";

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

    set({ guilds });
  },
  async leaveGuild(guildId) {
    await invoke("leave_guild", { guildId });
    set({ activeGuildId: null, activeChannelId: null });
    get().getGuilds();
  },

  async updateProfile(payload, crop) {
    await invoke("update_profile", { payload, crop });
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
