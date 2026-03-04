import { StateCreator } from "zustand";
import { AppStore, ChannelRepository } from "../types";

export const createChannelService: StateCreator<
  AppStore,
  [],
  [],
  ChannelRepository
> = (set) => ({
  createChannel: async (guildId, name, categoryId) => {
    console.log("createChannel stub", {
      guildId,
      name,
      categoryId
    });
  },
  deleteChannel: async (channelId) => {
    console.log("deleteChannel stub", channelId);
  },
  selectChannel: async (guildId, channelId) => {
    set({ activeGuildId: guildId, activeChannelId: channelId });
  }
});
