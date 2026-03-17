import { useState } from "react";
import { Cog, LogOut, MoreHorizontal } from "lucide-react";
import { Link } from "@tanstack/react-router";
import { Guild } from "@/types";
import { Button } from "@/components/ui/button";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { DropdownMenuItem } from "@/components/ui/dropdown-menu";
import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuSeparator,
  ContextMenuTrigger
} from "@/components/ui/context-menu";

type Props = {
  guild: Guild;
  isActive: boolean;
  onLeave: () => void;
  onSettings: () => void;
};

export function GuildRow({ guild, isActive, onLeave, onSettings }: Props) {
  const [hovered, setHovered] = useState(false);

  return (
    <ContextMenu>
      <ContextMenuTrigger
        render={
          <div
            className="relative flex items-center rounded-sm"
            onMouseEnter={() => setHovered(true)}
            onMouseLeave={() => setHovered(false)}
          >
            <DropdownMenuItem
              render={
                <Link
                  to="/g/$guildId/$channelId"
                  params={{
                    guildId: guild.id,
                    channelId: guild.default_channel_id!
                  }}
                  className="flex w-full items-center gap-2"
                >
                  <Avatar className="size-5 shrink-0 border-none">
                    <AvatarImage src={guild.icon_url ?? undefined} />
                    <AvatarFallback className="text-[10px]">
                      {guild.name.charAt(0)}
                    </AvatarFallback>
                  </Avatar>
                  <span className="truncate">{guild.name}</span>
                  {isActive && (
                    <span className="ml-auto size-1.5 rounded-full bg-primary shrink-0" />
                  )}
                </Link>
              }
              className="cursor-pointer flex-1 focus:text-white pr-8"
            />

            {hovered && (
              <Button
                variant="ghost"
                size="icon"
                className="absolute right-1 size-5 shrink-0 hover:bg-white/10"
                onPointerDown={(e) => {
                  e.stopPropagation();
                  e.preventDefault();
                }}
                onClick={(e) => {
                  e.stopPropagation();
                  e.preventDefault();
                  e.currentTarget.dispatchEvent(
                    new MouseEvent("contextmenu", {
                      bubbles: true,
                      clientX: e.clientX,
                      clientY: e.clientY
                    })
                  );
                }}
              >
                <MoreHorizontal className="size-3" />
              </Button>
            )}
          </div>
        }
      />

      <ContextMenuContent className="w-48 border-white/10">
        <ContextMenuItem onClick={onSettings} className="cursor-pointer">
          <Cog className="size-4" />
          <span>Guild Settings</span>
        </ContextMenuItem>
        <ContextMenuSeparator className="bg-white/10" />
        <ContextMenuItem
          onClick={onLeave}
          className="text-destructive focus:text-destructive cursor-pointer"
        >
          <LogOut className="size-4" />
          <span>Leave Guild</span>
        </ContextMenuItem>
      </ContextMenuContent>
    </ContextMenu>
  );
}
