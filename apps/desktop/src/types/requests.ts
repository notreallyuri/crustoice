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

export type UpdateProfilePayload = {
  display_name?: string;
  bio?: string;
  avatar_url?: string;
};

export type UpdateUsernamePayload = {
  password: string;
  username: string;
};

export type UpdateEmailPayload = {
  password: string;
  email: string;
};

export type UpdatePasswordPayload = {
  password: string;
  new_password: string;
};
