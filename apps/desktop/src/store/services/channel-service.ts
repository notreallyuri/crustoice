import { StateCreator } from "zustand";
import { AppStore, ChannelRepository } from "../types";

export const createChannelService: StateCreator<
  AppStore,
  [],
  [],
  ChannelRepository
> = (set) => ({
  createChannel: async (payload) => {
    console.log("createChannel stub", { payload });
  },
  deleteChannel: async (channelId) => {
    console.log("deleteChannel stub", channelId);
  },
  selectChannel: async (activeGuildId, activeChannelId) => {
    set({ activeGuildId, activeChannelId });
  }
});
