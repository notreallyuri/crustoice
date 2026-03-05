import { StateCreator } from "zustand";
import { AppStore, GuildRepository } from "../types";
import { invoke } from "@tauri-apps/api/core";
import { Guild } from "@/types";

export const createGuildService: StateCreator<
  AppStore,
  [],
  [],
  GuildRepository
> = (set, get, _) => ({
  createGuild: async (payload, iconPath) => {
    await invoke<Guild>("create_guild", { payload, iconPath });

    get().getGuilds();
  },
  deleteGuild: async (guildId) => {
    console.log("deleteGuild stub", guildId);
  },
  selectGuild: (guildId) => {
    set({ activeGuildId: guildId });
  }
});
