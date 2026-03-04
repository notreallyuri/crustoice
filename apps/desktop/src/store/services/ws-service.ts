import { StateCreator } from "zustand";
import { AppStore, WebSocketRepository } from "../types";

export const createWebSocketService: StateCreator<
  AppStore,
  [],
  [],
  WebSocketRepository
> = () => ({
  initWebSocket: async () => {
    console.log("initWebSocket stub");
  }
});
