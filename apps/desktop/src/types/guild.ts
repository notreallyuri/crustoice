import { CategoryId, ChannelId, GuildId, UserId } from "./ids";
import { UserPublic } from "./user";

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
}

export interface GuildMember {
  guild_id: GuildId;
  user_id: UserId;
  nickname: string | null;
  roles: string[];
  joined_at: string;
  data: UserPublic;
}

export interface Guild {
  id: GuildId;
  owner_id: UserId;
  name: string;
  icon_url: string | null;
  banner_url: string | null;

  default_channel_id: ChannelId;

  members: GuildMember[];
  categories: ChannelCategory[];
  channels: MessageChannel[];
}
