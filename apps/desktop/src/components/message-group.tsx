import { ChatMessage } from "@/types";
import { Avatar, AvatarFallback } from "./ui/avatar";
import { AvatarGif } from "./avatar-gif";
import { formatTime } from "@/lib/utils";
import { UserProfileCard } from "./layout/profile-card";
import { Popover, PopoverContent, PopoverTrigger } from "./ui/popover";
import { useState } from "react";

export function MessageGroup({
  messages,
  userCache
}: {
  messages: ChatMessage[];
  userCache: Record<string, any>;
}) {
  const [hovered, setHovered] = useState(false);

  const first = messages[0];
  const author = userCache[first.author_id];
  const displayName = author?.display_name ?? first.author_id;
  const avatarUrl = author?.avatar_url;

  return (
    <div
      className="flex gap-3 px-2 py-0.5 w-full rounded group"
      onMouseEnter={() => setHovered(true)}
      onMouseLeave={() => setHovered(false)}
    >
      <div className="relative">
        <Popover>
          <PopoverTrigger asChild>
            <Avatar className="size-9 mt-0.5 shrink-0 cursor-pointer">
              <AvatarGif
                animated={hovered}
                src={avatarUrl ?? undefined}
                alt=""
              />
              <AvatarFallback className="text-xs bg-primary/20 text-primary">
                {displayName.slice(0, 2).toUpperCase()}
              </AvatarFallback>
            </Avatar>
          </PopoverTrigger>
          <PopoverContent
            side="right"
            align="start"
            className="w-72 p-0 overflow-hidden"
          >
            {author && <UserProfileCard user={author} />}
          </PopoverContent>
        </Popover>
      </div>

      <div className="flex flex-col w-full gap-0.5 min-w-0">
        <div className="hover:bg-white/5">
          <div className="flex items-baseline gap-2">
            <span className="text-sm font-semibold text-white truncate">
              {displayName}
            </span>
            <span className="text-xs text-muted-foreground shrink-0">
              {formatTime(first.created_at)}
            </span>
          </div>
          <p className="text-sm text-[#dcddde]  wrap-break-words leading-relaxed">
            {first.content}
          </p>
        </div>
        {messages.slice(1).map((msg) => (
          <div key={msg.id} className="relative group/msg hover:bg-white/5">
            <span className="absolute -left-14 top-1/2 -translate-y-1/2 text-xs text-muted-foreground opacity-0 group-hover/msg:opacity-100 transition-opacity">
              {formatTime(msg.created_at)}
            </span>
            <p className="text-sm text-[#dcddde] wrap-break-words leading-relaxed">
              {msg.content}
            </p>
          </div>
        ))}
      </div>
    </div>
  );
}
