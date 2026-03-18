import { createFileRoute, useParams } from "@tanstack/react-router";
import { useAppStore } from "@/store/app-store";
import { useEffect, useRef, useState } from "react";
import { Message, Guild, Channel, TextChannel, VoiceChannel } from "@/types";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Textarea } from "@/components/ui/textarea";
import { FileText, Hash, Layers, MessageSquare, Volume2 } from "lucide-react";
import { MessageGroup } from "@/components/message-group";
import { groupMessages } from "@/lib/group-messages";

export const Route = createFileRoute("/g/$guildId/$channelId")({
  component: RouteComponent
});

const EMPTY: Message[] = [];

function RouteComponent() {
  const { guildId, channelId } = useParams({ from: "/g/$guildId/$channelId" });
  const guilds = useAppStore((s) => s.guilds);
  const messages = useAppStore((s) => s.messages[channelId] ?? EMPTY);
  const selectChannel = useAppStore((s) => s.selectChannel);
  const userCache = useAppStore((s) => s.userCache);
  const sendMessage = useAppStore((s) => s.sendMessage);

  const guild = guilds.find((g: Guild) => g.id === guildId);
  const channel = guild?.channels.find((c: Channel) => c.id === channelId);

  useEffect(() => {
    selectChannel(guildId, channelId);
  }, [guildId, channelId]);

  if (!channel) return null;

  switch (channel.kind) {
    case "text":
      return (
        <TextChannelView
          channel={channel}
          messages={messages}
          userCache={userCache}
          sendMessage={sendMessage}
        />
      );
    case "voice":
      return <VoiceChannelView channel={channel} />;
    case "docs":
      return <DocsChannelView channel={channel} />;
    case "canvas":
      return <CanvasChannelView channel={channel} />;
  }
}

function ChannelHeader({
  icon,
  name,
  children
}: {
  icon: React.ReactNode;
  name: string;
  children?: React.ReactNode;
}) {
  return (
    <div className="flex items-center gap-2 px-4 h-12 border-b border-border shrink-0">
      {icon}
      <span className="font-semibold text-sm">{name}</span>
      {children && (
        <div className="ml-auto flex items-center gap-2">{children}</div>
      )}
    </div>
  );
}

function TextChannelView({
  channel,
  messages,
  userCache,
  sendMessage
}: {
  channel: TextChannel & { kind: "text" };
  messages: Message[];
  userCache: Record<string, any>;
  sendMessage: (channelId: string, content: string) => void;
}) {
  switch (channel.mode) {
    case "board":
      return (
        <BoardView
          channel={channel}
          messages={messages}
          userCache={userCache}
          sendMessage={sendMessage}
        />
      );
    case "threads":
      return (
        <ThreadsView
          channel={channel}
          messages={messages}
          userCache={userCache}
          sendMessage={sendMessage}
        />
      );
    default:
      return (
        <ChatView
          channel={channel}
          messages={messages}
          userCache={userCache}
          sendMessage={sendMessage}
        />
      );
  }
}

function ChatView({
  channel,
  messages,
  userCache,
  sendMessage
}: {
  channel: TextChannel & { kind: "text" };
  messages: Message[];
  userCache: Record<string, any>;
  sendMessage: (channelId: string, content: string) => void;
}) {
  const [input, setInput] = useState("");
  const bottomRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: "instant" });
  }, [messages]);

  const handleMessage = () => {
    if (!input.trim()) return;
    sendMessage(channel.id, input);
    setInput("");
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      handleMessage();
    }
  };

  return (
    <div className="flex flex-col h-full w-full">
      <ChannelHeader
        icon={<Hash className="size-5 text-muted-foreground" />}
        name={channel.name}
      />

      <ScrollArea className="flex-1">
        <div className="flex flex-col gap-1 py-4">
          {messages.length === 0 && (
            <div className="flex flex-col items-center justify-center h-full py-16 text-muted-foreground">
              <Hash className="size-16 mb-4 opacity-20" />
              <p className="font-semibold text-white">
                Welcome to #{channel.name}
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

      <div className="px-4 pb-2 shrink-0">
        <div className="flex items-end rounded-none gap-2 bg-muted px-4 py-2">
          <Textarea
            value={input}
            onChange={(e) => setInput(e.target.value)}
            onKeyDown={handleKeyDown}
            placeholder={`Message #${channel.name}`}
            className="border-0 bg-transparent shadow-none focus-visible:ring-0 focus-visible:border-0 dark:bg-transparent p-0 text-sm"
            rows={1}
          />
        </div>
        <p className="text-[10px] text-muted-foreground mt-1 px-1">
          Enter to send · Shift+Enter for new line
        </p>
      </div>
    </div>
  );
}

function BoardView({
  channel,
  messages,
  userCache,
  sendMessage
}: {
  channel: TextChannel & { kind: "text" };
  messages: Message[];
  userCache: Record<string, any>;
  sendMessage: (channelId: string, content: string) => void;
}) {
  const [input, setInput] = useState("");

  const handlePost = () => {
    if (!input.trim()) return;
    sendMessage(channel.id, input);
    setInput("");
  };

  return (
    <div className="flex flex-col h-full w-full">
      <ChannelHeader
        icon={<Layers className="size-5 text-muted-foreground" />}
        name={channel.name}
      />

      <ScrollArea className="flex-1 px-4 py-4">
        {messages.length === 0 ? (
          <div className="flex flex-col items-center justify-center h-full py-16 text-muted-foreground">
            <Layers className="size-16 mb-4 opacity-20" />
            <p className="font-semibold text-white">No posts yet</p>
            <p className="text-sm">Be the first to post something.</p>
          </div>
        ) : (
          <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
            {messages.map((message) => {
              const author = userCache[message.author_id];
              return (
                <div
                  key={message.id}
                  className="rounded-lg border border-border bg-muted/30 p-4 flex flex-col gap-2"
                >
                  <p className="text-sm text-foreground">{message.content}</p>
                  <p className="text-xs text-muted-foreground mt-auto pt-2 border-t border-border/50">
                    {author?.display_name ?? message.author_id}
                  </p>
                </div>
              );
            })}
          </div>
        )}
      </ScrollArea>

      <div className="px-4 pb-2 shrink-0">
        <div className="flex items-end rounded-none gap-2 bg-muted px-4 py-2">
          <Textarea
            value={input}
            onChange={(e) => setInput(e.target.value)}
            onKeyDown={(e) => {
              if (e.key === "Enter" && !e.shiftKey) {
                e.preventDefault();
                handlePost();
              }
            }}
            placeholder="New post..."
            className="border-0 bg-transparent shadow-none focus-visible:ring-0 focus-visible:border-0 dark:bg-transparent p-0 text-sm"
            rows={1}
          />
        </div>
      </div>
    </div>
  );
}

function ThreadsView({
  channel,
  messages,
  userCache,
  sendMessage
}: {
  channel: TextChannel & { kind: "text" };
  messages: Message[];
  userCache: Record<string, any>;
  sendMessage: (channelId: string, content: string) => void;
}) {
  const [input, setInput] = useState("");

  const roots = messages.filter((m) => !m.thread_id);

  const replies = messages.reduce<Record<string, Message[]>>((acc, m) => {
    if (m.thread_id) {
      acc[m.thread_id] = [...(acc[m.thread_id] ?? []), m];
    }
    return acc;
  }, {});

  const handlePost = () => {
    if (!input.trim()) return;
    sendMessage(channel.id, input);
    setInput("");
  };

  return (
    <div className="flex flex-col h-full w-full">
      <ChannelHeader
        icon={<MessageSquare className="size-5 text-muted-foreground" />}
        name={channel.name}
      />

      <ScrollArea className="flex-1 px-4 py-4">
        {roots.length === 0 ? (
          <div className="flex flex-col items-center justify-center h-full py-16 text-muted-foreground">
            <MessageSquare className="size-16 mb-4 opacity-20" />
            <p className="font-semibold text-white">No threads yet</p>
            <p className="text-sm">Start a new thread below.</p>
          </div>
        ) : (
          <div className="flex flex-col gap-4">
            {roots.map((root) => {
              const author = userCache[root.author_id];
              const threadReplies = replies[root.id] ?? [];
              return (
                <div
                  key={root.id}
                  className="rounded-lg border border-border bg-muted/20 p-4 flex flex-col gap-3"
                >
                  <div className="flex items-start gap-2">
                    <div className="flex-1">
                      <p className="text-xs font-semibold text-muted-foreground mb-1">
                        {author?.display_name ?? root.author_id}
                      </p>
                      <p className="text-sm text-foreground">{root.content}</p>
                    </div>
                  </div>

                  {threadReplies.length > 0 && (
                    <div className="border-l-2 border-border/50 pl-3 flex flex-col gap-2">
                      {threadReplies.map((reply) => {
                        const replyAuthor = userCache[reply.author_id];
                        return (
                          <div key={reply.id}>
                            <p className="text-xs font-semibold text-muted-foreground">
                              {replyAuthor?.display_name ?? reply.author_id}
                            </p>
                            <p className="text-sm text-foreground">
                              {reply.content}
                            </p>
                          </div>
                        );
                      })}
                    </div>
                  )}

                  <p className="text-xs text-muted-foreground/50">
                    {threadReplies.length}{" "}
                    {threadReplies.length === 1 ? "reply" : "replies"}
                  </p>
                </div>
              );
            })}
          </div>
        )}
      </ScrollArea>

      <div className="px-4 pb-2 shrink-0">
        <div className="flex items-end rounded-none gap-2 bg-muted px-4 py-2">
          <Textarea
            value={input}
            onChange={(e) => setInput(e.target.value)}
            onKeyDown={(e) => {
              if (e.key === "Enter" && !e.shiftKey) {
                e.preventDefault();
                handlePost();
              }
            }}
            placeholder="Start a new thread..."
            className="border-0 bg-transparent shadow-none focus-visible:ring-0 focus-visible:border-0 dark:bg-transparent p-0 text-sm"
            rows={1}
          />
        </div>
      </div>
    </div>
  );
}

function VoiceChannelView({
  channel
}: {
  channel: VoiceChannel & { kind: "voice" };
}) {
  return (
    <div className="flex flex-col h-full w-full">
      <ChannelHeader
        icon={<Volume2 className="size-5 text-muted-foreground" />}
        name={channel.name}
      >
        {channel.user_limit && (
          <span className="text-xs text-muted-foreground">
            {channel.participants.length} / {channel.user_limit}
          </span>
        )}
      </ChannelHeader>

      <div className="flex flex-col items-center justify-center flex-1 gap-3 text-muted-foreground">
        <Volume2 className="size-12 opacity-20" />
        <p className="text-sm font-medium">Voice coming soon</p>
        <p className="text-xs opacity-60">
          {channel.participants.length === 0
            ? "No one here yet"
            : `${channel.participants.length} participant${channel.participants.length !== 1 ? "s" : ""}`}
        </p>
      </div>
    </div>
  );
}

function DocsChannelView({ channel }: { channel: Channel & { kind: "docs" } }) {
  return (
    <div className="flex flex-col h-full w-full">
      <ChannelHeader
        icon={<FileText className="size-5 text-muted-foreground" />}
        name={channel.name}
      />
      <div className="flex flex-col items-center justify-center flex-1 gap-3 text-muted-foreground">
        <FileText className="size-12 opacity-20" />
        <p className="text-sm font-medium">Docs coming soon</p>
      </div>
    </div>
  );
}

function CanvasChannelView({
  channel
}: {
  channel: Channel & { kind: "canvas" };
}) {
  return (
    <div className="flex flex-col h-full w-full">
      <ChannelHeader
        icon={<Layers className="size-5 text-muted-foreground" />}
        name={channel.name}
      />
      <div className="flex flex-col items-center justify-center flex-1 gap-3 text-muted-foreground">
        <Layers className="size-12 opacity-20" />
        <p className="text-sm font-medium">Canvas coming soon</p>
      </div>
    </div>
  );
}
