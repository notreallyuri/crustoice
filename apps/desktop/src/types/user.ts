import { UserId } from "./ids";

export type PresenceStatus =
  | "Online"
  | "Away"
  | "Busy"
  | "Invisible"
  | "Offline";

export type PresenceTimer =
  | "Elapsed"
  | { Countdown: { seconds: number } }
  | "Off";

export type PresenceKind = "Fixed" | { AppLinked: { process_name: string } };

export type Emoji =
  | { Unicode: { value: string } }
  | { Custom: { id: string; name: string; url: string } };

export type PresenceIcon =
  | { CustomUpload: { path_url: string } }
  | { Emoji: { emoji: Emoji } }
  | { AppIcon: { process_name: string } };

export interface PresencePreset {
  id: string;
  label: string;
  icon: PresenceIcon;
  timer: PresenceTimer;
  kind: PresenceKind;
}

export type UIDarkMode = "system" | "light" | "dark";
export type UITheme = "default" | "strawberry" | "blueberry";
export type Locale = "en-US" | "pt-BR";
export type ActivityKind = "playing" | "streaming" | "listening" | "watching";
export type RelationshipStatus =
  | "None"
  | "Friend"
  | "Blocked"
  | "PendingIncoming"
  | "PendingOutcoming";

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
  preset: PresencePreset | null;
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
  presence_presets: PresencePreset[];
}

export interface UserRelationship {
  id: UserId;
  user: UserPublic;
  status: RelationshipStatus;
  since: string;
}

export interface UserPublic {
  id: UserId;
  username: string;
  display_name: string;
  avatar_url: string | null;
  bio: string | null;
  presence: UserPresence;
}
