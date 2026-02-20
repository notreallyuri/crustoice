import { StateCreator } from "zustand";
import { AppStore, ChannelRepository } from "../types";
import { invoke } from "@tauri-apps/api/core";
import { MessageChannel, ChatMessage } from "@/types";

export const channelService: StateCreator<
  AppStore,
  [],
  [],
  ChannelRepository
> = (set) => ({
  createChannel: async (guildId, name, categoryId = null) => {
    const newChannel = await invoke<MessageChannel>("create_channel", {
      guildId,
      name,
      categoryId
    });

    set((state) => ({
      guilds: state.guilds.map((g) =>
        g.id === guildId ? { ...g, channels: [...g.channels, newChannel] } : g
      )
    }));
  },
  deleteChannel: async (channelId) => {
    await invoke("delete_channel", { channelId });
    set((state) => {
      const updatedGuilds = state.guilds.map((g) => ({
        ...g,
        channels: g.channels.filter((c) => c.id !== channelId)
      }));

      let nextActiveChannelId = state.activeChannelId;

      if (state.activeChannelId === channelId) {
        nextActiveChannelId = null;

        if (state.activeGuildId) {
          const activeGuild = updatedGuilds.find(
            (g) => g.id === state.activeGuildId
          );
          if (activeGuild && activeGuild.channels.length > 0) {
            nextActiveChannelId = activeGuild.channels[0].id;
          }
        }
      }

      return {
        guilds: updatedGuilds,
        activeChannelId: nextActiveChannelId
      };
    });
  },
  selectChannel: async (guildId, channelId) => {
    set({ activeGuildId: guildId, activeChannelId: channelId });
    try {
      const history = await invoke<ChatMessage[]>("get_messages", {
        channelId
      });
      set((state) => ({
        messages: { ...state.messages, [channelId]: history }
      }));
    } catch (e) {
      console.error(e);
    }
  }
});
