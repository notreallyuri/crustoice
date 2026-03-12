import { createFileRoute, useParams } from "@tanstack/react-router";
import { useAppStore } from "@/store/app-store";
import { useEffect, useRef, useState } from "react";
import { ChatMessage, Guild, MessageChannel } from "@/types";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Textarea } from "@/components/ui/textarea";
import { Hash } from "lucide-react";
import { MessageGroup } from "@/components/message-group";
import { groupMessages } from "@/lib/group-messages";

export const Route = createFileRoute("/g/$guildId/$channelId")({
  component: RouteComponent
});

function RouteComponent() {
  const { guildId, channelId } = useParams({ from: "/g/$guildId/$channelId" });

  const guilds = useAppStore((s) => s.guilds);
  const EMPTY: ChatMessage[] = [];
  const messages = useAppStore((s) => s.messages[channelId] ?? EMPTY);
  const selectChannel = useAppStore((s) => s.selectChannel);
  const ws = useAppStore((s) => s.ws);
  const userCache = useAppStore((s) => s.userCache);

  const [input, setInput] = useState("");
  const bottomRef = useRef<HTMLDivElement>(null);

  const guild = guilds.find((g: Guild) => g.id === guildId);
  const channel = guild?.channels.find(
    (c: MessageChannel) => c.id === channelId
  );

  useEffect(() => {
    selectChannel(guildId, channelId);
  }, [guildId, channelId]);

  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: "instant" });
  }, [messages]);

  const sendMessage = () => {
    const content = input.trim();
    if (!content || !ws || ws.readyState !== WebSocket.OPEN) return;

    ws.send(
      JSON.stringify({
        type: "Chat",
        channel_id: channelId,
        content
      })
    );
    setInput("");
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      sendMessage();
    }
  };

  return (
    <div className="flex flex-col h-full w-full">
      <div className="flex items-center gap-2 px-4 h-12 border-b border-border shrink-0">
        <Hash className="size-5 text-muted-foreground" />
        <span className="font-semibold text-sm">
          {channel?.name ?? channelId}
        </span>
      </div>

      <ScrollArea className="flex-1 px-4">
        <div className="flex flex-col gap-1 py-4">
          {messages.length === 0 && (
            <div className="flex flex-col items-center justify-center h-full py-16 text-muted-foreground">
              <Hash className="size-16 mb-4 opacity-20" />
              <p className="font-semibold text-white">
                Welcome to #{channel?.name ?? channelId}
              </p>
              <p className="text-sm">This is the beginning of this channel.</p>
            </div>
          )}
          {groupMessages(messages).map((group) => (
            <MessageGroup
              key={group[0].id}
              messages={group}
              userCache={userCache}
            />
          ))}
          <div ref={bottomRef} />
        </div>
      </ScrollArea>

      <div className="px-4 pb-6 shrink-0">
        <div className="flex items-end gap-2 bg-muted px-4 py-2">
          <Textarea
            value={input}
            onChange={(e) => setInput(e.target.value)}
            onKeyDown={handleKeyDown}
            placeholder={`Message #${channel?.name ?? channelId}`}
            className="border-0 bg-transparent shadow-none focus-visible:ring-0 focus-visible:border-0 dark:bg-transparent p-0 text-sm"
            rows={1}
          />
        </div>
        <p className="text-xs text-muted-foreground mt-1 px-1">
          Enter to send · Shift+Enter for new line
        </p>
      </div>
    </div>
  );
}
