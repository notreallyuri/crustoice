import { StateCreator } from "zustand";
import { AppStore, GuildRepository } from "../types";
import { invoke } from "@tauri-apps/api/core";
import { Guild } from "@/types";

export const guildService: StateCreator<AppStore, [], [], GuildRepository> = (
  set,
  get,
) => ({
  createGuild: async (name) => {
    const ownerId = get().currentUser?.id;
    if (!ownerId) return;
    const newGuild = await invoke<Guild>("create_guild", { name, ownerId });
    set((state) => ({
      guilds: [...state.guilds, newGuild],
      activeGuildId: newGuild.id,
      activeChannelId: newGuild.channels[0]?.id || null,
    }));
  },
  deleteGuild: async (guildId) => {
    await invoke("delete_guild", { guildId });
    set((state) => ({
      guilds: state.guilds.filter((g) => g.id !== guildId),
      activeGuildId:
        state.activeGuildId === guildId ? null : state.activeGuildId,
      activeChannelId:
        state.activeGuildId === guildId ? null : state.activeChannelId,
    }));
  },
  selectGuild: (guildId) => {
    const guild = get().guilds.find((g) => g.id === guildId);
    if (!guild) return;

    const defaultChannelId = guild.channels[0]?.id || null;
    set({ activeGuildId: guildId });

    if (defaultChannelId) {
      get().selectChannel(guildId, defaultChannelId);
    }
  },
});
