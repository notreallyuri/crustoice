import {
  ChevronDown,
  Cog,
  Home,
  Plus,
  LogOut,
  MoreHorizontal
} from "lucide-react";
import { Button } from "../ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger
} from "../ui/dropdown-menu";
import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuSeparator,
  ContextMenuTrigger
} from "../ui/context-menu";
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle
} from "../ui/alert-dialog";
import { Link, useNavigate } from "@tanstack/react-router";
import { Guild } from "@/types";
import { SidebarHeader as ShadSidebarHeader } from "@/components/ui/sidebar";
import { AvatarFallback, Avatar, AvatarImage } from "../ui/avatar";
import { useAppStore } from "@/store/app-store";
import { useState } from "react";
import { toast } from "sonner";

type Props = {
  guilds: Guild[];
  activeGuild: Guild | undefined;
  isHome: boolean;
  setCreateDialogOpen: (v: boolean) => void;
};

type GuildActionTarget = Guild | null;

export function SidebarHeader({
  guilds,
  activeGuild,
  isHome,
  setCreateDialogOpen
}: Props) {
  const leaveGuild = useAppStore((s) => s.leaveGuild);
  const navigate = useNavigate();
  const [leaveDialogOpen, setLeaveDialogOpen] = useState(false);
  const [isLeaving, setIsLeaving] = useState(false);
  const [actionTarget, setActionTarget] = useState<GuildActionTarget>(null);

  const handleLeaveGuild = async () => {
    if (!actionTarget) return;
    setIsLeaving(true);
    try {
      await leaveGuild(actionTarget.id);
      navigate({ to: "/g/@me" });
      toast.success(`Left ${actionTarget.name}`);
    } catch (e) {
      toast.error("Failed to leave guild", { description: String(e) });
    } finally {
      setIsLeaving(false);
      setLeaveDialogOpen(false);
      setActionTarget(null);
    }
  };

  const openLeaveDialog = (guild: Guild) => {
    setActionTarget(guild);
    setLeaveDialogOpen(true);
  };

  const triggerLabel = isHome
    ? "Home"
    : (activeGuild?.name ?? "Select a Guild");

  return (
    <>
      <ShadSidebarHeader className="border-b border-black/20 p-0">
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button
              variant="ghost"
              className="h-12 w-full justify-between rounded-none px-3 text-white hover:bg-white/5 focus-visible:ring-0 focus-visible:ring-offset-0"
            >
              <div className="flex items-center gap-2 min-w-0">
                {!isHome && activeGuild && (
                  <Avatar className="size-5 rounded-sm shrink-0">
                    <AvatarImage src={activeGuild.icon_url ?? undefined} />
                    <AvatarFallback className="text-[10px] rounded-sm bg-primary/20">
                      {activeGuild.name.charAt(0)}
                    </AvatarFallback>
                  </Avatar>
                )}
                {isHome && (
                  <Home className="size-4 shrink-0 text-muted-foreground" />
                )}
                <span className="truncate font-semibold text-sm">
                  {triggerLabel}
                </span>
              </div>
              <ChevronDown className="size-4 shrink-0 text-muted-foreground" />
            </Button>
          </DropdownMenuTrigger>

          <DropdownMenuContent
            className="w-60 border-white/10 shadow-xl"
            align="center"
            sideOffset={4}
          >
            <DropdownMenuItem
              asChild
              className="cursor-pointer group focus:text-white"
            >
              <Link to="/g/@me" className="flex w-full items-center gap-2">
                <Home className="size-4 text-muted-foreground group-hover:text-primary transition-colors" />
                <span>Home</span>
              </Link>
            </DropdownMenuItem>

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
      </ShadSidebarHeader>

      <AlertDialog open={leaveDialogOpen} onOpenChange={setLeaveDialogOpen}>
        <AlertDialogContent size="sm">
          <AlertDialogHeader>
            <AlertDialogTitle>Leave {actionTarget?.name}?</AlertDialogTitle>
            <AlertDialogDescription>
              You'll need an invite to rejoin. This cannot be undone.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel disabled={isLeaving}>Cancel</AlertDialogCancel>
            <AlertDialogAction
              onClick={handleLeaveGuild}
              disabled={isLeaving}
              className="bg-destructive hover:bg-destructive/90 text-destructive-foreground"
            >
              {isLeaving ? (
                <div className="size-4 border-2 border-current border-t-transparent rounded-full animate-spin" />
              ) : (
                "Leave Guild"
              )}
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </>
  );
}

type GuildRowProps = {
  guild: Guild;
  isActive: boolean;
  onLeave: () => void;
  onSettings: () => void;
};

function GuildRow({ guild, isActive, onLeave, onSettings }: GuildRowProps) {
  const [hovered, setHovered] = useState(false);

  return (
    <ContextMenu>
      <ContextMenuTrigger asChild>
        <div
          className="relative flex items-center rounded-sm"
          onMouseEnter={() => setHovered(true)}
          onMouseLeave={() => setHovered(false)}
        >
          <DropdownMenuItem
            asChild
            className="cursor-pointer flex-1 focus:text-white pr-8"
          >
            <Link
              to="/g/$guildId/$channelId"
              params={{
                guildId: guild.id,
                channelId: guild.default_channel_id
              }}
              className="flex w-full items-center gap-2"
            >
              <Avatar className="size-5 rounded-sm shrink-0 border border-white/10">
                <AvatarImage src={guild.icon_url ?? undefined} />
                <AvatarFallback className="text-[10px] rounded-sm">
                  {guild.name.charAt(0)}
                </AvatarFallback>
              </Avatar>
              <span className="truncate">{guild.name}</span>
              {isActive && (
                <span className="ml-auto size-1.5 rounded-full bg-primary shrink-0" />
              )}
            </Link>
          </DropdownMenuItem>

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
                // Simulate right-click to open context menu
                const event = new MouseEvent("contextmenu", {
                  bubbles: true,
                  clientX: e.clientX,
                  clientY: e.clientY
                });
                e.currentTarget.dispatchEvent(event);
              }}
            >
              <MoreHorizontal className="size-3" />
            </Button>
          )}
        </div>
      </ContextMenuTrigger>

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
