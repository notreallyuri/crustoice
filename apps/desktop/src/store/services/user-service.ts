import { StateCreator } from "zustand";
import { AppStore, UserRepository } from "../types";
import { invoke } from "@tauri-apps/api/core";
import { Guild, UserProfile } from "@/types";

export const userService: StateCreator<AppStore, [], [], UserRepository> = (
  set
) => ({
  login: async (username) => {
    const user = await invoke<UserProfile>("login", { username });
    set({ currentUser: user });

    try {
      const guilds = await invoke<Guild[]>("get_guilds", { userId: user.id });
      set({ guilds });
    } catch (e) {
      console.error("Failed to fetch initial guilds:", e);
    }
  },
  sendMessage: async (content) => {
    set((state) => {
      if (!state.activeChannelId) return {};
      invoke("send_chat", { content, channelId: state.activeChannelId });
      return {};
    });
  }
});
