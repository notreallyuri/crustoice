import { ChannelId, MessageId, UserId } from "./ids";

export interface ChatMessage {
  id: MessageId;
  channel_id: ChannelId;
  author_id: UserId;
  content: string;
  created_at: string;
  edited_at: string | null;
}
