import { StateCreator } from "zustand";
import { AppStore, ChannelRepository } from "../types";
import { invoke } from "@tauri-apps/api/core";
import { Message, Channel, ChannelCategory } from "@/types";

export const createChannelService: StateCreator<
  AppStore,
  [],
  [],
  ChannelRepository
> = (set, get) => ({
  createChannel: async (payload) => {
    const activeGuildId = get().activeGuildId;

    if (!activeGuildId) return;

    await invoke<Channel>("create_channel", {
      guildId: activeGuildId,
      payload
    });
  },
  createCategory: async (payload) => {
    const activeGuildId = get().activeGuildId;

    if (!activeGuildId) return;

    await invoke<ChannelCategory>("create_category", {
      guildId: activeGuildId,
      payload
    });
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
