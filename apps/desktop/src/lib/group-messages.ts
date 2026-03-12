import { ChatMessage } from "@/types";

export function groupMessages(messages: ChatMessage[]): ChatMessage[][] {
  const groups: ChatMessage[][] = [];
  let current: ChatMessage[] = [];

  for (const msg of messages) {
    const last = current[current.length - 1];
    const timeDiff = last
      ? Number(msg.created_at) - Number(last.created_at)
      : 0;
    const sameAuthor = last?.author_id === msg.author_id;
    const withinWindow = timeDiff < 10 * 60 * 1000; // 10 minutes in ms

    if (current.length === 0 || (sameAuthor && withinWindow)) {
      current.push(msg);
    } else {
      groups.push(current);
      current = [msg];
    }
  }

  if (current.length > 0) groups.push(current);
  return groups;
}
