import { CategoryId, GuildId } from "./ids";

export type RegisterPayload = {
  email: string;
  username: string;
  password: string;
  display_name?: string;
};

export type LoginPayload = {
  email: string;
  password: string;
};

export type CreateChannelPayload = {
  guildId: GuildId;
  name: string;
  categoryId: CategoryId | null;
};

export type CreateGuilldPayload = {
  name: string;
};
