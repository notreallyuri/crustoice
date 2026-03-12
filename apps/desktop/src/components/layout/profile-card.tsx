import { cn } from "@/lib/utils";
import { UserPublic } from "@/types";
import { Avatar, AvatarFallback } from "../ui/avatar";
import { AtSign, UserIcon } from "lucide-react";
import { AvatarGif } from "../avatar-gif";

type Props = {
  user: UserPublic;
  className?: string;
};

export function UserProfileCard({ user, className }: Props) {
  return (
    <div className={cn("w-72 overflow-hidden", className)}>
      <div className="h-16 bg-linear-to-br from-primary/40 via-primary/20 to-transparent" />
      <div className="px-4 pb-4">
        <div className="-mt-8 mb-3">
          <Avatar className="size-16 border-4 border-popover ring-1 ring-border/50">
            {user.avatar_url ? (
              <AvatarGif
                src={user.avatar_url}
                alt={user.display_name}
                animated
              />
            ) : (
              <AvatarFallback className="bg-red-700/90 text-lg">
                <UserIcon size={24} />
              </AvatarFallback>
            )}
          </Avatar>
        </div>
        <div className="mb-3">
          <p className="font-semibold text-sm leading-tight">
            {user.display_name}
          </p>
          <p className="text-muted-foreground text-xs inline-flex items-center gap-0.5 mt-0.5">
            <AtSign className="size-3" />
            {user.username}
          </p>
        </div>
        {user.bio ? (
          <div className="rounded-md bg-muted/60 border border-border/30 px-3 py-2">
            <p className="text-xs leading-relaxed text-foreground/80">
              {user.bio}
            </p>
          </div>
        ) : (
          <div className="rounded-md bg-muted/30 border border-dashed border-border/30 px-3 py-2">
            <p className="text-xs text-muted-foreground/60 italic">
              No bio provided.
            </p>
          </div>
        )}
      </div>
    </div>
  );
}
