import { CategoryId } from "./ids";
import { ChannelMode } from "./guild";

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

export type CreateChannelPayload =
  | {
      kind: "text";
      name: string;
      category_id: CategoryId | null;
      mode: ChannelMode;
    }
  | {
      kind: "voice";
      name: string;
      category_id: CategoryId | null;
      bitrate?: number;
      user_limit?: number;
    }
  | {
      kind: "docs";
      name: string;
      category_id: CategoryId | null;
    }
  | {
      kind: "canvas";
      name: string;
      category_id: CategoryId | null;
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
  current_password: string;
  new_username: string;
};

export type UpdateEmailPayload = {
  current_password: string;
  new_email: string;
};

export type UpdatePasswordPayload = {
  current_password: string;
  new_password: string;
};
