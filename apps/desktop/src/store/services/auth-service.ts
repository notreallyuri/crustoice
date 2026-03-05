import { StateCreator } from "zustand";
import { AppStore, AuthRepository } from "../types";
import { invoke } from "@tauri-apps/api/core";

export const createAuthService: StateCreator<
  AppStore,
  [],
  [],
  AuthRepository
> = (_, get) => ({
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

  async register(payload, avatarPath) {
    await invoke<string>("register", {
      payload,
      avatarPath
    });

    await get().getMe();
  },

  async updateAvatar(file) {
    try {
      const extension = file.name.split(".").pop()?.toLowerCase();

      if (!extension) throw new Error("Invalid file name");

      const uploadUrl = await invoke<string>("get_avatar_upload_url", {
        extension
      });

      const uploadRes = await fetch(uploadUrl, {
        method: "PUT",
        body: file,
        headers: {
          "Content-Type": file.type
        }
      });

      if (!uploadRes.ok) throw new Error("R2 Upload failed");

      await invoke("confirm_avatar_upload", { extension });

      await get().getMe();
      console.log("Avatar updated successfully!");
    } catch (e) {
      console.error("Avatar update failed:", e);
      throw e;
    }
  }
});
