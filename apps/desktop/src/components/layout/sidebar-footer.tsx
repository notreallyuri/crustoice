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

export function SidebarFooter({ currentUser }: { currentUser: User | null }) {
  const [muted, setMuted] = useState(false);
  const [deafened, setDeafened] = useState(false);

  return (
    <ShadSidebarFooter className="h-15 flex-row items-center justify-between p-2 shadow-md">
      <div className="flex border w-full pr-1">
        <div className="flex flex-1 cursor-pointer items-center gap-2 overflow-hidden rounded-md px-2 py-1 transition-colors hover:bg-white/10">
          <Avatar>
            <AvatarImage src={currentUser?.profile.avatar_url ?? undefined} />
            <AvatarFallback>
              <UserIcon size={24} className="-mb-2" />
            </AvatarFallback>
          </Avatar>
          <div className="flex flex-col truncate leading-tight">
            <span className="truncate text-[13px] font-semibold text-white">
              {currentUser?.profile.display_name}
            </span>
            <span className="truncate text-[11px] text-muted-foreground">
              {currentUser?.profile.username}
            </span>
          </div>
        </div>

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
