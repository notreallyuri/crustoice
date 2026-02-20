import { CategoryId, ChannelId, GuildId, UserId } from "./ids";
import { ChatMessage } from "./intents";

export interface ChannelCategory {
  id: CategoryId;
  guild_id: GuildId;
  name: string;
  position: number;
}

export interface MessageChannel {
  id: ChannelId;
  guild_id: GuildId;
  category_id: CategoryId | null;
  name: string;
  position: number;
  history: ChatMessage[];
}

export interface GuildMember {
  guild_id: GuildId;
  user_id: UserId;
  nickname: string | null;
  roles: string[];
  joined_at: string;
}

export interface Guild {
  id: GuildId;
  owner_id: UserId;
  name: string;

  members: UserId[];
  categories: ChannelCategory[];
  channels: MessageChannel[];
}
