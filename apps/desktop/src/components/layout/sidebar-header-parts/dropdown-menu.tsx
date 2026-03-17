import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger
} from "@/components/ui/dropdown-menu";
import { Guild } from "@/types";
import { Link } from "@tanstack/react-router";
import { Home, Plus } from "lucide-react";
import { GuildRow } from "./guild-row";
import { ReactElement } from "react";

type Props = {
  guilds: Guild[];
  activeGuild: Guild | undefined;
  setCreateDialogOpen: (v: boolean) => void;
  children: ReactElement;
  openLeaveDialog: (guild: Guild) => void;
};

export function HeaderDropdownMenu({
  activeGuild,
  setCreateDialogOpen,
  guilds,
  openLeaveDialog,
  children
}: Props) {
  return (
    <DropdownMenu>
      <DropdownMenuTrigger render={children} />
      <DropdownMenuContent
        className="w-60 border-white/10 shadow-xl"
        align="center"
        sideOffset={4}
      >
        <DropdownMenuItem
          render={
            <Link to="/g/@me" className="flex w-full items-center gap-2">
              <Home className="size-4 text-muted-foreground group-hover:text-primary transition-colors" />
              <span>Home</span>
            </Link>
          }
          className="cursor-pointer group focus:text-white"
        />

        {guilds.length > 0 && (
          <>
            <DropdownMenuSeparator className="bg-white/10" />
            <p className="px-2 py-1 text-[10px] font-bold uppercase tracking-wider text-muted-foreground/50">
              Your Guilds
            </p>
            {guilds.map((guild) => (
              <GuildRow
                key={guild.id}
                guild={guild}
                isActive={guild.id === activeGuild?.id}
                onLeave={() => openLeaveDialog(guild)}
                onSettings={() => {
                  /* TODO */
                }}
              />
            ))}
          </>
        )}

        <DropdownMenuSeparator className="bg-white/10" />
        <DropdownMenuItem
          onClick={() => setCreateDialogOpen(true)}
          className="cursor-pointer"
        >
          <Plus className="size-4" />
          <span>Create Guild</span>
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
