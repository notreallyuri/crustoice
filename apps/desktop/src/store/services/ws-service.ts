import { StateCreator } from "zustand";
import { AppStore, WebSocketRepository } from "../types";
import { invoke } from "@tauri-apps/api/core";
import { ServerMessage } from "@/types/protocol";
import { UserPublic } from "@/types";

export const createWebSocketService: StateCreator<
  AppStore,
  [],
  [],
  WebSocketRepository
> = (set, get) => ({
  async initWebSocket() {
    const token = await invoke<string>("get_token");

    return new Promise<void>((resolve, reject) => {
      const ws = new WebSocket("ws://127.0.0.1:3000/ws");

      ws.onopen = () => {
        ws.send(JSON.stringify({ type: "Identify", token }));
      };

      ws.onmessage = (event) => {
        try {
          const msg = JSON.parse(event.data) as ServerMessage;
          handleServerMessage(msg, set, get, ws);

          if (msg.type === "IdentityValidated") {
            resolve();
          }
        } catch (e) {
          console.error("Failed to parse WS message:", e);
        }
      };

      ws.onerror = () => {
        console.error("WebSocket error");
      };

      ws.onclose = (event) => {
        if (event.code === 4001) {
          console.error("Identity failed, clearing session");
          reject(new Error("Identity failed"));
          get().logout();
          return;
        }
        console.warn("WebSocket disconnected, reconnecting...");
        setTimeout(() => get().initWebSocket(), 3000);
      };

      set({ ws });
    });
  },
  setPresence(presence) {
    const ws = get().ws;
    if (!ws || ws.readyState !== WebSocket.OPEN) return;
    ws.send(JSON.stringify({ type: "SetPresence", presence }));

    const currentUser = get().currentUser;
    if (currentUser) {
      set({ currentUser: { ...currentUser, presence } });
    }
  },
  sendMessage(content) {}
});

function handleServerMessage(
  msg: ServerMessage,
  set: {
    (
      partial:
        | AppStore
        | Partial<AppStore>
        | ((state: AppStore) => AppStore | Partial<AppStore>),
      replace?: false
    ): void;
    (state: AppStore | ((state: AppStore) => AppStore), replace: true): void;
  },
  _get: () => AppStore,
  _ws: WebSocket
) {
  switch (msg.type) {
    case "IdentityValidated": {
      set({ currentUser: msg.user });

      break;
    }

    case "InitialState": {
      const userCache: Record<string, UserPublic> = {};
      for (const guild of msg.guilds) {
        for (const member of guild.members) {
          userCache[member.user_id] = member.data;
        }
      }
      set({ guilds: msg.guilds, userCache });
      break;
    }

    case "GuildJoined": {
      set((state: AppStore) => ({
        guilds: [...state.guilds, msg.guild]
      }));
      break;
    }

    case "MemberJoined": {
      set((state: AppStore) => ({
        guilds: state.guilds.map((guild) =>
          guild.id === msg.guild_id
            ? { ...guild, members: [...guild.members, msg.member] }
            : guild
        )
      }));
      break;
    }

    case "Message": {
      const { message } = msg;
      set((state: any) => {
        const existing = state.messages[message.channel_id] ?? [];
        const isDuplicate = existing.some((m: any) => m.id === message.id);
        if (isDuplicate) return state;

        return {
          messages: {
            ...state.messages,
            [message.channel_id]: [...existing, message]
          }
        };
      });
      break;
    }

    case "PresenceUpdate": {
      const { user } = msg;

      set((state: AppStore) => {
        const updatedCache = {
          ...state.userCache,
          [user.id]: user
        };

        const updatedGuilds = state.guilds.map((guild) => ({
          ...guild,
          members: guild.members.map((member) =>
            member.user_id === user.id ? { ...member, data: user } : member
          )
        }));

        const currentUser = state.currentUser;
        const updatedCurrentUser =
          currentUser && currentUser.id === user.id
            ? { ...currentUser, presence: user.presence }
            : currentUser;

        return {
          userCache: updatedCache,
          guilds: updatedGuilds,
          currentUser: updatedCurrentUser
        };
      });

      break;
    }

    case "Error":
      console.error("WS server error:", msg.message);
      break;
  }
}
