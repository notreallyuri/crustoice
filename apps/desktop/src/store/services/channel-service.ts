import { StateCreator } from "zustand";
import { AppStore, ChannelRepository } from "../types";
import { invoke } from "@tauri-apps/api/core";
import { Message, Channel } from "@/types";

export const createChannelService: StateCreator<
  AppStore,
  [],
  [],
  ChannelRepository
> = (set, get) => ({
  createChannel: async (payload) => {
    const activeGuildId = get().activeGuildId;

    if (!activeGuildId) return;

    const channel = await invoke<Channel>("create_channel", {
      guildId: activeGuildId,
      payload
    });

    set((state: AppStore) => ({
      guilds: state.guilds.map((g) =>
        g.id === activeGuildId
          ? { ...g, channels: [...g.channels, channel] }
          : g
      )
    }));
  },
  deleteChannel: async (channelId) => {
    console.log("deleteChannel stub", channelId);
  },
  selectChannel: async (guildId, channelId, threadId) => {
    set({ activeGuildId: guildId, activeChannelId: channelId });

    if (get().messages[channelId] !== undefined) return;

    try {
      const messages = await invoke<Message[]>("get_channel_history", {
        channelId,
        threadId: threadId ?? null
      });
      set((state) => ({
        messages: { ...state.messages, [channelId]: messages }
      }));
    } catch (e) {
      console.error("Failed to fetch history:", e);
    }
  }
});
