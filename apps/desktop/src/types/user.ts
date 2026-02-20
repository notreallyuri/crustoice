import { UserId } from "./ids";

export interface UserProfile {
  id: UserId;
  username: string;
  display_name: string;
  pfp: string | null;
  description: string | null;
}
