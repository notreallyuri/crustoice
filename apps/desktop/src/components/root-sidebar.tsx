import { cn } from "@/lib/utils";
import { ChannelId, Guild, GuildId, UserProfile } from "@/types";
import { Hash, Server } from "lucide-react";
import { useState } from "react";
import { DialogCreateGuild } from "./dialog-create-guild";
import { Separator } from "./ui/separator";
import { Button } from "./ui/button";
import { ScrollArea } from "./ui/scroll-area";

type Props = {
  guilds: Guild[];
  channelAction: (guildId: GuildId, channelId: ChannelId) => Promise<void>;
  activeGuildId: string | null;
  activeChannelId: string | null;
  activeGuild: Guild | undefined;
  currentUser: UserProfile | null;
};

export function RootSidebar({
  guilds,
  channelAction,
  activeGuildId,
  currentUser,
  activeGuild,
  activeChannelId
}: Props) {
  const [isGuildsShown, setIsGuildsShown] = useState(true);

  return (
    <>
      <nav
        className={cn(
          "flex flex-col items-center gap-2 pt-3 px-2 overflow-x-hidden bg-background",
          isGuildsShown ? "w-18" : "w-0"
        )}
      >
        {guilds.map((guild) => (
          <button
            key={guild.id}
            onClick={() => {
              if (guild.channels.length > 0) {
                channelAction(guild.id, guild.channels[0].id);
              }
            }}
            className={`group relative flex h-12 w-12 items-center justify-center rounded-[24px] bg-[#313338] transition-all duration-200 hover:rounded-3xl hover:bg-[#5865f2] ${
              activeGuildId === guild.id ? "rounded-3xl bg-[#5865f2]" : ""
            }`}
          >
            <Server className="h-6 w-6" />
            {activeGuildId === guild.id && (
              <div className="absolute -left-3 h-10 w-2 rounded-r-full bg-white" />
            )}
          </button>
        ))}

        <Separator />

        <DialogCreateGuild />
      </nav>
      <aside className="bg-sidebar flex w-60 flex-col">
        <div className="flex h-12 items-center px-4 shadow-sm font-bold truncate">
          {activeGuild?.name || "Select a Server"}
        </div>

        <ScrollArea className="flex-1 px-2 pt-3">
          <div className="space-y-0.5">
            {activeGuild?.channels.map((channel) => (
              <Button
                key={channel.id}
                variant={activeChannelId === channel.id ? "secondary" : "ghost"}
                className={`w-full justify-start gap-2 px-2 text-zinc-400 hover:bg-[#35373c] hover:text-zinc-200 ${
                  activeChannelId === channel.id
                    ? "bg-[#3f4147] text-white"
                    : ""
                }`}
                onClick={() => channelAction(activeGuild.id, channel.id)}
              >
                <Hash className="h-4 w-4 opacity-50" />
                <span className="truncate">{channel.name}</span>
              </Button>
            ))}
          </div>
        </ScrollArea>
        <div className="flex items-center gap-2 bg-sidebar border-t border-sidebar-border p-2">
          <div className="h-8 w-8 shrink-0 rounded-full bg-[#5865f2] flex items-center justify-center font-bold text-xs">
            {currentUser?.username[0].toUpperCase()}
          </div>
          <div className="flex flex-col min-w-0">
            <span className="text-xs font-bold truncate">
              {currentUser?.username}
            </span>
            <span className="text-[10px] text-zinc-400"># Online</span>
          </div>
        </div>
      </aside>
    </>
  );
}
