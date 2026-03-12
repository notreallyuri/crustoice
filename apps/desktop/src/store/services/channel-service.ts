import { StateCreator } from "zustand";
import { AppStore, ChannelRepository } from "../types";
import { invoke } from "@tauri-apps/api/core";
import { ChatMessage } from "@/types";

export const createChannelService: StateCreator<
  AppStore,
  [],
  [],
  ChannelRepository
> = (set, get) => ({
  createChannel: async (payload) => {
    console.log("createChannel stub", { payload });
  },
  deleteChannel: async (channelId) => {
    console.log("deleteChannel stub", channelId);
  },
  selectChannel: async (guildId, channelId) => {
    set({ activeGuildId: guildId, activeChannelId: channelId });

    if (get().messages[channelId]?.length) return;

    try {
      const messages = await invoke<ChatMessage[]>("get_channel_history", {
        channelId
      });
      set((state) => ({
        messages: { ...state.messages, [channelId]: messages }
      }));
    } catch (e) {
      console.error("Failed to fetch history:", e);
    }
  }
});
