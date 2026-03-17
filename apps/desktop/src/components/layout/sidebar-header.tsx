import { useState } from "react";
import {
  ChevronDown,
  Cog,
  Hash,
  Home,
  LogOut,
  Plus,
  Users,
  Mail
} from "lucide-react";
import { Button } from "../ui/button";
import { Guild } from "@/types";
import { SidebarHeader as ShadSidebarHeader } from "@/components/ui/sidebar";
import { Avatar, AvatarFallback, AvatarImage } from "../ui/avatar";
import { Tooltip, TooltipContent, TooltipTrigger } from "../ui/tooltip";
import { cn } from "@/lib/utils";
import { LeaveGuildDialog } from "./sidebar-header-parts/dialog-leave-guild";
import { HeaderDropdownMenu } from "./sidebar-header-parts/dropdown-menu";
import { DialogCreateMenu } from "../dialogs/create-menu/dialog-main";
import { DialogCreateGuild } from "../dialogs/dialog-create-guild";
import { DialogCreateChannel } from "../dialogs/create-menu/dialog-create-channel";

export type Tab = "channels" | "members";
export type CreateDialogOptions = "menu" | "channel" | "event" | null;

type Props = {
  guilds: Guild[];
  activeGuild: Guild | undefined;
  isHome: boolean;
  activeTab: Tab;
  setActiveTab: (tab: Tab) => void;
};

export function SidebarHeader({
  guilds,
  activeGuild,
  isHome,
  activeTab,
  setActiveTab
}: Props) {
  const [leaveDialogOpen, setLeaveDialogOpen] = useState(false);
  const [createGuildDialogOpen, setCreateGuildDialogOpen] = useState(false);

  const [createDialog, setCreateDialog] = useState<CreateDialogOptions>(null);

  const [actionTarget, setActionTarget] = useState<Guild | null>(null);

  const openLeaveDialog = (guild: Guild) => {
    setActionTarget(guild);
    setLeaveDialogOpen(true);
  };

  return (
    <>
      <ShadSidebarHeader className="flex px-0">
        <HeaderDropdownMenu
          guilds={guilds}
          activeGuild={activeGuild}
          setCreateDialogOpen={setCreateGuildDialogOpen}
          openLeaveDialog={openLeaveDialog}
        >
          <Button
            variant="ghost"
            className="h-8 w-full justify-between px-3 text-white"
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
                {isHome ? "Home" : (activeGuild?.name ?? "Select a Guild")}
              </span>
            </div>
            <ChevronDown className="size-4 shrink-0 text-muted-foreground" />
          </Button>
        </HeaderDropdownMenu>
      </ShadSidebarHeader>

      {!isHome && activeGuild && (
        <div className="flex flex-col ">
          <div className="flex items-center w-full gap-2 pt-1.5 mb-1.5">
            <Tooltip>
              <TooltipTrigger
                delay={100}
                className="flex-1"
                render={
                  <Button
                    variant="outline"
                    size="sm"
                    onClick={() => setCreateDialog("menu")}
                  >
                    <Plus className="size-3.5" />
                  </Button>
                }
              />
              <TooltipContent side="bottom" align="start">
                Create
              </TooltipContent>
            </Tooltip>

            <Tooltip>
              <TooltipTrigger
                delay={100}
                className="flex-1"
                render={
                  <Button variant="outline" size="sm">
                    <Mail className="size-3.5" />
                  </Button>
                }
              />
              <TooltipContent side="bottom" align="center">
                Invites
              </TooltipContent>
            </Tooltip>

            <Tooltip>
              <TooltipTrigger
                delay={100}
                className="flex-1"
                render={
                  <Button variant="outline" size="sm">
                    <Cog className="size-3.5" />
                  </Button>
                }
              />
              <TooltipContent side="bottom" align="center">
                Guild Settings
              </TooltipContent>
            </Tooltip>

            <Tooltip>
              <TooltipTrigger
                delay={100}
                className="flex-1"
                render={
                  <Button
                    variant="outline"
                    size="sm"
                    onClick={() => openLeaveDialog(activeGuild)}
                  >
                    <LogOut className="size-3.5" />
                  </Button>
                }
              />
              <TooltipContent side="bottom" align="end">
                Leave Guild
              </TooltipContent>
            </Tooltip>
          </div>

          <div className="flex px-1 mb-1.5 py-1 rounded-sm gap-1 bg-neutral-950 items-center">
            {(["channels", "members"] as Tab[]).map((tab) => (
              <button
                key={tab}
                type="button"
                onClick={() => setActiveTab(tab)}
                className={cn(
                  "flex flex-1 items-center justify-center gap-1.5 rounded-md py-1 text-xs font-medium transition-colors",
                  activeTab === tab
                    ? "bg-white/10 text-white"
                    : "text-muted-foreground hover:text-white hover:bg-white/5"
                )}
              >
                {tab === "channels" ? (
                  <Hash className="size-3" />
                ) : (
                  <Users className="size-3" />
                )}
                {tab.charAt(0).toUpperCase() + tab.slice(1)}
              </button>
            ))}
          </div>
        </div>
      )}

      <DialogCreateMenu
        open={createDialog === "menu"}
        onOpenChange={(v) => setCreateDialog(v ? "menu" : null)}
        onCreateChannel={() => setCreateDialog("channel")}
        onCreateEvent={() => setCreateDialog("event")}
      />

      <DialogCreateChannel
        open={createDialog === "channel"}
        onOpenChange={(v) => setCreateDialog(v ? "channel" : null)}
        categories={activeGuild?.categories ?? []}
        goBack={() => setCreateDialog("menu")}
      />

      <LeaveGuildDialog
        open={leaveDialogOpen}
        onOpenChange={setLeaveDialogOpen}
        target={actionTarget}
      />
      <DialogCreateGuild
        isOpen={createGuildDialogOpen}
        setIsOpen={setCreateGuildDialogOpen}
      />
    </>
  );
}
