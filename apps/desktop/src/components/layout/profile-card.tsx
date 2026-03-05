import { cn } from "@/lib/utils";
import { User } from "@/types";
import { Avatar, AvatarFallback, AvatarImage } from "../ui/avatar";
import { AtSign, UserIcon } from "lucide-react";
import { useClickOutside } from "@/hooks/use-click-outside";

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
        className={cn(
          "absolute bottom-full mb-2 z-50 min-w-62 bg-popover text-popover-foreground rounded-md border p-3 shadow-xl animate-in fade-in zoom-in-95 duration-100",
          className
        )}
      >
        <div
          className="flex items-center gap-2"
          onClick={(e) => e.stopPropagation()}
        >
          <Avatar className="rounded-none size-10">
            <AvatarImage
              className="rounded-none"
              src={user.profile.avatar_url ?? undefined}
            />
            <AvatarFallback className="rounded-none ">
              <UserIcon size={32} />
            </AvatarFallback>
          </Avatar>
          <div>
            <p className="text-sm">{user.profile.display_name}</p>
            <p className="text-muted-foreground items-center text-xs inline-flex">
              <AtSign className="size-3 mr-1" /> {user.profile.username}
            </p>
          </div>
        </div>
        <div className="border-border/20 border text-sm p-1 bg-muted mt-2">
          {user.profile.bio ? (
            <p>{user.profile.bio}</p>
          ) : (
            <p className="text-muted-foreground">No bio provided</p>
          )}
        </div>
      </div>
    </>
  );
}
