import { cn } from "@/lib/utils";
import { User } from "@/types";
import { Avatar, AvatarFallback } from "../ui/avatar";
import { AtSign, UserIcon } from "lucide-react";
import { useClickOutside } from "@/hooks/use-click-outside";
import { AvatarGif } from "../avatar-gif";

type Props = {
  user: User;
  onClose: () => void;
  className?: string;
};

export function UserProfileCard({ user, onClose, className }: Props) {
  const cardRef = useClickOutside(onClose);

  return (
    <>
      <div
        className="fixed inset-0 z-49 bg-transparent cursor-default"
        onClick={(e) => {
          e.stopPropagation();
          onClose();
        }}
      />

      <div
        ref={cardRef}
        onClick={(e) => e.stopPropagation()}
        className={cn(
          "absolute bottom-full z-50 w-72 bg-popover text-popover-foreground rounded-lg border border-border/50 shadow-2xl animate-in fade-in zoom-in-95 duration-100 overflow-hidden",
          className
        )}
      >
        <div className="h-16 bg-linear-to-br from-primary/40 via-primary/20 to-transparent" />

        <div className="px-4 pb-4">
          <div className="-mt-8 mb-3">
            <Avatar className="size-16 border-4 border-popover ring-1 ring-border/50">
              {user.profile.avatar_url ? (
                <AvatarGif
                  src={user.profile.avatar_url}
                  alt={user.profile.display_name}
                />
              ) : (
                <AvatarFallback className="bg-primary/20 text-lg">
                  <UserIcon size={24} />
                </AvatarFallback>
              )}
            </Avatar>
          </div>

          <div className="mb-3">
            <p className="font-semibold text-sm leading-tight">
              {user.profile.display_name}
            </p>
            <p className="text-muted-foreground text-xs inline-flex items-center gap-0.5 mt-0.5">
              <AtSign className="size-3" />
              {user.profile.username}
            </p>
          </div>

          {user.profile.bio ? (
            <div className="rounded-md bg-muted/60 border border-border/30 px-3 py-2">
              <p className="text-xs leading-relaxed text-foreground/80">
                {user.profile.bio}
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
    </>
  );
}
