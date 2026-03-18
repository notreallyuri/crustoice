import { StateCreator } from "zustand";
import { AppStore, WebSocketRepository } from "../types";
import { invoke } from "@tauri-apps/api/core";
import { ServerMessage } from "@/types/protocol";
import { UserPublic } from "@/types";
import { listen } from "@tauri-apps/api/event";

export const createWebSocketService: StateCreator<
  AppStore,
  [],
  [],
  WebSocketRepository
> = (set, get) => ({
  async initWebSocket() {
    await listen<ServerMessage>("ws://message", (event) => {
      handleServerMessage(event.payload, set);
    });
  },

  setPresence(presence) {
    invoke("ws_send", {
      message: { type: "SetPresence", presence }
    });

    const currentUser = get().currentUser;
    if (currentUser) {
      set({ currentUser: { ...currentUser, presence } });
    }
  },

  sendMessage(channel_id, content) {
    invoke("ws_send", {
      message: { type: "Chat", channel_id, content: content.trim() }
    });
  },

  editMessage(channelId, messageId, content) {
    invoke("ws_send", {
      message: {
        type: "EditMessage",
        channel_id: channelId,
        message_id: messageId,
        content
      }
    });
  },

  deleteMessage(channelId, messageId) {
    invoke("ws_send", {
      message: {
        type: "DeleteMessage",
        channel_id: channelId,
        message_id: messageId
      }
    });
  }
});

function handleServerMessage(
  msg: ServerMessage,
  set: (
    partial:
      | AppStore
      | Partial<AppStore>
      | ((state: AppStore) => AppStore | Partial<AppStore>)
  ) => void
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
      set((state: AppStore) => {
        const existing = state.messages[message.channel_id] ?? [];
        const isDuplicate = existing.some((m) => m.id === message.id);
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

    case "MessageEdited": {
      const { message } = msg;
      set((state: AppStore) => ({
        messages: {
          ...state.messages,
          [message.channel_id]: (state.messages[message.channel_id] ?? []).map(
            (m) => (m.id === message.id ? message : m)
          )
        }
      }));
      break;
    }

    case "MessageDeleted": {
      const { channel_id, message_id } = msg;
      set((state: AppStore) => ({
        messages: {
          ...state.messages,
          [channel_id]: (state.messages[channel_id] ?? []).filter(
            (m) => m.id !== message_id
          )
        }
      }));
      break;
    }

    case "PresenceUpdate": {
      const { user } = msg;
      set((state: AppStore) => {
        const updatedCache = { ...state.userCache, [user.id]: user };
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
