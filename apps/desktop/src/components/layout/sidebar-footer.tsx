import { SidebarFooter as ShadSidebarFooter } from "@/components/ui/sidebar";
import { User, UserPublic } from "@/types";
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
import { Avatar, AvatarBadge, AvatarFallback } from "@/components/ui/avatar";
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
  const [muted, setMuted] = useState(false);
  const [deafened, setDeafened] = useState(false);

  function publicFromUser(user: User): UserPublic {
    return {
      id: user.id,
      username: user.profile.username,
      display_name: user.profile.display_name,
      avatar_url: user.profile.avatar_url,
      banner_url: user.profile.banner_url,
      profile_color: user.profile.profile_color,
      bio: user.profile.bio,
      presence: user.presence
    };
  }

  return (
    <ShadSidebarFooter className="flex-row items-center justify-between p-0  relative">
      <div className="border flex pr-2 w-full">
        <UserProfileCard
          side="top"
          align="start"
          className="mt-2"
          user={publicFromUser(currentUser)}
          showPresenceAction
        >
          <button
            className="flex flex-1 min-w-0 cursor-pointer items-center gap-2.5 px-2 py-1.5 transition-colors hover:bg-white/8 group"
            type="button"
          >
            <div className="relative ">
              <Avatar>
                {currentUser.profile.avatar_url && (
                  <AvatarGif src={currentUser.profile.avatar_url} alt="" />
                )}
                <AvatarFallback>
                  <UserIcon size={16} />
                </AvatarFallback>
                <AvatarBadge status={currentUser.presence.status} />
              </Avatar>
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
        </UserProfileCard>
        <div className="flex shrink-0 items-center gap-0.5 ml-1">
          <Tooltip>
            <TooltipTrigger
              render={
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
              }
            />
            <TooltipContent side="top">
              {muted ? "Unmute" : "Mute"}
            </TooltipContent>
          </Tooltip>

          <Tooltip>
            <TooltipTrigger
              render={
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
              }
            />
            <TooltipContent side="top">
              {deafened ? "Undeafen" : "Deafen"}
            </TooltipContent>
          </Tooltip>

          <Tooltip>
            <TooltipTrigger
              render={
                <Button
                  variant="ghost"
                  size="icon"
                  className="size-7 text-muted-foreground hover:bg-white/10 hover:text-white cursor-pointer"
                  onClick={() => setSettingsDialogOpen(true)}
                >
                  <Settings className="size-4" />
                </Button>
              }
            />
            <TooltipContent side="top">Settings</TooltipContent>
          </Tooltip>
        </div>
      </div>
    </ShadSidebarFooter>
  );
}
