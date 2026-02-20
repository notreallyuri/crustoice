import { ChatMessage, Guild, UserProfile } from "@/types";
import { StateCreator } from "zustand";
import { AppStore, WebSocketRepository } from "../types";
import { listen } from "@tauri-apps/api/event";

type WsPayload =
  | { type: "IdentityValidated"; user: UserProfile }
  | { type: "InitialState"; connected_users: UserProfile[]; guilds: Guild[] }
  | { type: "PresenceUpdate"; user: UserProfile; status: string }
  | { type: "Message"; message: ChatMessage }
  | { type: "Error"; code: string; message: string };

export const webSocketService: StateCreator<
  AppStore,
  [],
  [],
  WebSocketRepository
> = (set) => ({
  initWebSocket: async () => {
    await listen<string>("ws-event", (event) => {
      const payload: WsPayload = JSON.parse(event.payload);

      switch (payload.type) {
        case "InitialState":
          set({
            guilds: payload.guilds,
            userCache: Object.fromEntries(
              payload.connected_users.map((u) => [u.id, u])
            )
          });
          break;
        case "Message":
          const msg = payload.message;
          set((state) => ({
            messages: {
              ...state.messages,
              [msg.channel_id]: [...(state.messages[msg.channel_id] || []), msg]
            }
          }));
          break;
        case "PresenceUpdate":
          set((state) => ({
            userCache: { ...state.userCache, [payload.user.id]: payload.user }
          }));
          break;
        case "IdentityValidated":
          set({ currentUser: payload.user });
          break;
      }
    });
  }
});
