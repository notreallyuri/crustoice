import { SidebarFooter as ShadSidebarFooter } from "@/components/ui/sidebar";
import { User } from "@/types";
import { Button } from "../ui/button";
import { useState } from "react";
import {
  HeadphoneOff,
  Headphones,
  Mic,
  MicOff,
  Settings,
  User as UserIcon
} from "lucide-react";
import { Avatar, AvatarFallback } from "@/components/ui/avatar";
import { UserProfileCard } from "./profile-card";
import { AvatarGif } from "../avatar-gif";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger
} from "@/components/ui/tooltip";
import { cn } from "@/lib/utils";

type Props = {
  currentUser: User;
  setSettingsDialogOpen: (open: boolean) => void;
};

export function SidebarFooter({ currentUser, setSettingsDialogOpen }: Props) {
  const [profileShow, setProfileShow] = useState(false);
  const [muted, setMuted] = useState(false);
  const [deafened, setDeafened] = useState(false);

  return (
    <ShadSidebarFooter className="h-14 flex-row items-center justify-between px-2 py-1.5 border-t border-border/50 bg-background/80 backdrop-blur-sm relative">
      {profileShow && (
        <UserProfileCard
          user={currentUser}
          onClose={() => setProfileShow(false)}
          className="mb-2"
        />
      )}

      <button
        onClick={(e) => {
          e.preventDefault();
          setProfileShow(true);
        }}
        className="flex flex-1 min-w-0 cursor-pointer items-center gap-2.5 rounded-md px-2 py-1.5 transition-colors hover:bg-white/8 group"
      >
        <div className="relative shrink-0">
          <Avatar className="size-8">
            {currentUser.profile.avatar_url ? (
              <AvatarGif src={currentUser.profile.avatar_url} alt="" />
            ) : (
              <AvatarFallback className="bg-primary/20">
                <UserIcon size={16} />
              </AvatarFallback>
            )}
          </Avatar>
          {/* Online indicator */}
          <span className="absolute bottom-0 right-0 size-2.5 rounded-full bg-green-500 border-2 border-background" />
        </div>

        <div className="flex flex-col truncate leading-tight text-left">
          <span className="truncate text-sm font-semibold text-foreground group-hover:text-white transition-colors">
            {currentUser.profile.display_name}
          </span>
          <span className="truncate text-xs text-muted-foreground">
            @{currentUser.profile.username}
          </span>
        </div>
      </button>

      <div className="flex shrink-0 items-center gap-0.5 ml-1">
        <Tooltip>
          <TooltipTrigger asChild>
            <Button
              variant="ghost"
              size="icon"
              className={cn(
                "size-7 text-muted-foreground hover:bg-white/10 hover:text-white cursor-pointer transition-colors",
                muted && "text-destructive hover:text-destructive"
              )}
              onClick={() => setMuted((m) => !m)}
            >
              {muted ? (
                <MicOff className="size-4" />
              ) : (
                <Mic className="size-4" />
              )}
            </Button>
          </TooltipTrigger>
          <TooltipContent side="top">
            {muted ? "Unmute" : "Mute"}
          </TooltipContent>
        </Tooltip>

        <Tooltip>
          <TooltipTrigger asChild>
            <Button
              variant="ghost"
              size="icon"
              className={cn(
                "size-7 text-muted-foreground hover:bg-white/10 hover:text-white cursor-pointer transition-colors",
                deafened && "text-destructive hover:text-destructive"
              )}
              onClick={() => setDeafened((d) => !d)}
            >
              {deafened ? (
                <HeadphoneOff className="size-4" />
              ) : (
                <Headphones className="size-4" />
              )}
            </Button>
          </TooltipTrigger>
          <TooltipContent side="top">
            {deafened ? "Undeafen" : "Deafen"}
          </TooltipContent>
        </Tooltip>

        <Tooltip>
          <TooltipTrigger asChild>
            <Button
              variant="ghost"
              size="icon"
              className="size-7 text-muted-foreground hover:bg-white/10 hover:text-white cursor-pointer"
              onClick={() => setSettingsDialogOpen(true)}
            >
              <Settings className="size-4" />
            </Button>
          </TooltipTrigger>
          <TooltipContent side="top">Settings</TooltipContent>
        </Tooltip>
      </div>
    </ShadSidebarFooter>
  );
}
