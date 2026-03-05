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
  async sendMessage(content) {
    const { activeChannelId } = get();
    if (!activeChannelId) return;

    await invoke("send_chat", {
      channelId: activeChannelId,
      content
    });
  },

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
  }
});
