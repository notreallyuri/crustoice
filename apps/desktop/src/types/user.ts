import { UserId } from "./ids";

export type PresenceStatus =
  | "online"
  | "away"
  | "busy"
  | "invisible"
  | "offline";
export type UIDarkMode = "system" | "light" | "dark";
export type UITheme = "default" | "strawberry" | "blueberry";
export type Locale = "en-US" | "pt-BR";
export type ActivityKind = "playing" | "streaming" | "listening" | "watching";

export interface User {
  id: UserId;
  account: UserAccount;
  profile: UserProfile;
  settings: UserSettings;
  presence: UserPresence;
}

export interface UserAccount {
  email: string;
  verified: boolean;
}

export interface UserProfile {
  username: string;
  display_name: string;
  avatar_url: string | null;
  bio: string | null;
}

export interface UserPresence {
  status: PresenceStatus;
  custom_message: string | null;
}

export interface UserActivity {
  name: string;
  kind: ActivityKind;
}

export interface UserSettings {
  ui: {
    dark_mode: UIDarkMode;
    theme: UITheme;
  };
  notifications: {
    active: boolean;
  };
  locale: Locale;
}
