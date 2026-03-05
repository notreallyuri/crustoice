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
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { UserProfileCard } from "./profile-card";

export function SidebarFooter({ currentUser }: { currentUser: User }) {
  const [profileShow, setProfileShow] = useState(false);
  const [muted, setMuted] = useState(false);
  const [deafened, setDeafened] = useState(false);

  return (
    <ShadSidebarFooter className="h-15 flex-row items-center justify-between p-2 shadow-md relative">
      {profileShow && (
        <UserProfileCard
          user={currentUser}
          onClose={() => setProfileShow(false)}
          className="w-[calc(100%-16px)]"
        />
      )}

      <div className="flex border w-full pr-1">
        <button
          onClick={(e) => {
            e.preventDefault();
            setProfileShow(true);
          }}
          className="flex flex-1 cursor-pointer items-center gap-2 overflow-hidden rounded-md px-2 py-1 transition-colors hover:bg-white/10"
        >
          <Avatar className="rounded-none">
            <AvatarImage
              className="rounded-none"
              src={currentUser?.profile.avatar_url ?? undefined}
            />
            <AvatarFallback className="rounded-none ">
              <UserIcon size={24} />
            </AvatarFallback>
          </Avatar>
          <div className="flex flex-col truncate leading-tight">
            <span className="truncate text-sm font-semibold text-white">
              {currentUser?.profile.display_name}
            </span>
            <span className="truncate text-xs text-muted-foreground">
              {currentUser?.profile.username}
            </span>
          </div>
        </button>

        <div className="flex shrink-0 items-center gap-0.5">
          <Button
            variant="ghost"
            size="icon"
            className="size-7 text-muted-foreground hover:bg-white/10 hover:text-white cursor-pointer"
            onClick={() => setMuted((m) => !m)}
          >
            {muted ? <MicOff className="text-destructive" /> : <Mic />}
          </Button>
          <Button
            variant="ghost"
            size="icon"
            className="size-7 text-muted-foreground hover:bg-white/10 hover:text-white cursor-pointer"
            onClick={() => setDeafened((d) => !d)}
          >
            {deafened ? (
              <HeadphoneOff className="text-destructive" />
            ) : (
              <Headphones />
            )}
          </Button>
          <Button
            variant="ghost"
            size="icon"
            className="size-7 text-muted-foreground hover:bg-white/10 hover:text-white cursor-pointer"
          >
            <Settings className="size-4" />
          </Button>
        </div>
      </div>
    </ShadSidebarFooter>
  );
}
