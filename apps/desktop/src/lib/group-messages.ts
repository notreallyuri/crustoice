import { Message } from "@/types";

export function groupMessages(messages: Message[]): Message[][] {
  const groups: Message[][] = [];
  let current: Message[] = [];

  for (const msg of messages) {
    if (current.length === 0) {
      current.push(msg);
      continue;
    }

    const last = current[current.length - 1];

    const msgTime = new Date(msg.created_at).getTime();
    const lastTime = new Date(last.created_at).getTime();

    const timeDiff = msgTime - lastTime;
    const sameAuthor = last.author_id === msg.author_id;
    const withinWindow = timeDiff < 10 * 60 * 1000;

    if (sameAuthor && withinWindow) {
      current.push(msg);
    } else {
      groups.push(current);
      current = [msg];
    }
  }

  if (current.length > 0) groups.push(current);

  return groups;
}
