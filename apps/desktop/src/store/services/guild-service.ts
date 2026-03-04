import { StateCreator } from "zustand";
import { AppStore, GuildRepository } from "../types";

export const createGuildService: StateCreator<
  AppStore,
  [],
  [],
  GuildRepository
> = (set, get, _) => ({
  createGuild: async (name) => {
    console.log("createGuild stub", name);

    get().getGuilds();
  },
  deleteGuild: async (guildId) => {
    console.log("deleteGuild stub", guildId);
  },
  selectGuild: (guildId) => {
    set({ activeGuildId: guildId });
  }
});
