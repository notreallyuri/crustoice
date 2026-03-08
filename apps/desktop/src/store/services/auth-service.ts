import { StateCreator } from "zustand";
import { AppStore, AuthRepository } from "../types";
import { invoke } from "@tauri-apps/api/core";
import { toast } from "sonner";

export const createAuthService: StateCreator<
  AppStore,
  [],
  [],
  AuthRepository
> = (set, get) => ({
  async initSession() {
    try {
      if (get().currentUser) return;

      await invoke("check_auth");

      await get().getMe();
      await get().getGuilds();

      console.log("Session restored successfully.");
    } catch (e) {
      console.log("Failed to restore session:", e);
      throw e;
    }
  },

  async login(payload) {
    await invoke<string>("login", {
      payload
    });

    await get().getMe();
    await get().getGuilds();
  },

  async register(payload, avatarPath, crop) {
    await invoke<string>("register", {
      payload,
      avatarPath,
      crop
    });

    await get().getMe();
  },

  async logout() {
    try {
      await invoke("logout");

      set({
        currentUser: null,
        guilds: [],
        messages: {},
        userCache: {},
        activeChannelId: null,
        activeGuildId: null
      });
    } catch (e) {
      toast.error("Logout failed. Please try again.");
      set({
        currentUser: undefined,
        activeGuildId: null,
        activeChannelId: null,
        guilds: []
      });
    }
  }
});
