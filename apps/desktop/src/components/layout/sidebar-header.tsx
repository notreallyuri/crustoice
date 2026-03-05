import { ChevronDown, Home, Plus } from "lucide-react";
import { Button } from "../ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger
} from "../ui/dropdown-menu";
import { Link } from "@tanstack/react-router";
import { Guild } from "@/types";
import { SidebarHeader as ShadSidebarHeader } from "@/components/ui/sidebar";

type Props = {
  guilds: Guild[];
  activeGuild: Guild | undefined;
  isHome: boolean;
  setCreateDialogOpen: (v: boolean) => void;
};

export function SidebarHeader({
  guilds,
  activeGuild,
  isHome,
  setCreateDialogOpen
}: Props) {
  return (
    <ShadSidebarHeader className="border-b border-black/20 p-0">
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button
            variant="ghost"
            className="h-12 w-full justify-between rounded-none px-4 text-white hover:bg-white/5 focus-visible:ring-0 focus-visible:ring-offset-0"
          >
            <span className="truncate font-semibold">
              {isHome ? "Home" : activeGuild?.name || "Select Context"}
            </span>
            <ChevronDown className="size-4 text-muted-foreground" />
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent
          className="w-56 border-white/10 text-white shadow-xl"
          align="center"
          sideOffset={4}
        >
          <DropdownMenuItem asChild className="cursor-pointer focus:text-white">
            <Link to="/g/@me" className="flex w-full items-center">
              <Home className="mr-2 size-4" />
              <span>Home</span>
            </Link>
          </DropdownMenuItem>

          {guilds.length > 0 && (
            <DropdownMenuSeparator className="bg-white/10" />
          )}

          {guilds.map((guild) => {
            console.log("Guild: ", guild);
            return (
              <DropdownMenuItem
                key={guild.id}
                asChild
                className="cursor-pointer focus:text-white"
              >
                <Link
                  to={`/g/${guild.id}/${guild.channels[0]}`}
                  className="flex w-full items-center"
                >
                  <div className="mr-2 flex size-5 items-center justify-center rounded bg-white/10 text-[10px] font-bold">
                    {guild.name.charAt(0)}
                  </div>
                  <span className="truncate">{guild.name}</span>
                </Link>
              </DropdownMenuItem>
            );
          })}
          <DropdownMenuSeparator className="bg-white/10" />
          <DropdownMenuItem
            onClick={() => setCreateDialogOpen(true)}
            className="cursor-pointer"
          >
            <Plus />
            Create Guild
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    </ShadSidebarHeader>
  );
}
