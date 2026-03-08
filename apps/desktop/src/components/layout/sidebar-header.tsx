import { Castle, ChevronDown, Cog, Home, Plus, LogOut } from "lucide-react";
import { Button } from "../ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
  DropdownMenuSubTrigger,
  DropdownMenuSub,
  DropdownMenuSubContent
} from "../ui/dropdown-menu";
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

  const handleLeaveGuild = async () => {
    if (!activeGuild) return;
    setIsLeaving(true);
    try {
      await leaveGuild(activeGuild.id);
      navigate({ to: "/g/@me" });
      toast.success(`Left ${activeGuild.name}`);
    } catch (e) {
      toast.error("Failed to leave guild", { description: String(e) });
    } finally {
      setIsLeaving(false);
      setLeaveDialogOpen(false);
    }
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

            {/* Guild list */}
            {guilds.length > 0 && (
              <>
                <DropdownMenuSeparator className="bg-white/10" />
                <p className="px-2 py-1 text-[10px] font-bold uppercase tracking-wider text-muted-foreground/50">
                  Your Guilds
                </p>
                {guilds.map((guild) => (
                  <DropdownMenuItem
                    key={guild.id}
                    asChild
                    className="cursor-pointer group focus:text-white"
                  >
                    <Link
                      to="/g/$guildId/$channelId"
                      params={{
                        guildId: guild.id,
                        channelId: guild.default_channel_id
                      }}
                      className="flex w-full items-center gap-2"
                    >
                      <Avatar className="size-5 rounded-sm shrink-0 border border-white/10 group-hover:border-primary/50 transition-colors">
                        <AvatarImage src={guild.icon_url ?? undefined} />
                        <AvatarFallback className="text-[10px] rounded-sm">
                          {guild.name.charAt(0)}
                        </AvatarFallback>
                      </Avatar>
                      <span className="truncate">{guild.name}</span>
                      {guild.id === activeGuild?.id && (
                        <span className="ml-auto size-1.5 rounded-full bg-primary shrink-0" />
                      )}
                    </Link>
                  </DropdownMenuItem>
                ))}
              </>
            )}

            {activeGuild && (
              <>
                <DropdownMenuSeparator className="bg-white/10" />
                <DropdownMenuSub>
                  <DropdownMenuSubTrigger className="cursor-pointer">
                    <Castle className="size-4" />
                    <span>Guild Options</span>
                  </DropdownMenuSubTrigger>
                  <DropdownMenuSubContent className="border-white/10">
                    <DropdownMenuItem className="cursor-pointer">
                      <Cog className="size-4" />
                      <span>General Settings</span>
                    </DropdownMenuItem>
                    <DropdownMenuSeparator className="bg-white/10" />
                    <DropdownMenuItem
                      className="text-destructive focus:text-destructive cursor-pointer"
                      onClick={() => setLeaveDialogOpen(true)}
                    >
                      <LogOut className="size-4" />
                      <span>Leave Guild</span>
                    </DropdownMenuItem>
                  </DropdownMenuSubContent>
                </DropdownMenuSub>
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
            <AlertDialogTitle>Leave {activeGuild?.name}?</AlertDialogTitle>
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
