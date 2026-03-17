import { useState } from "react";
import { useNavigate } from "@tanstack/react-router";
import { Guild } from "@/types";
import { useAppStore } from "@/store/app-store";
import { toast } from "sonner";
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle
} from "@/components/ui/alert-dialog";

type Props = {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  target: Guild | null;
};

export function LeaveGuildDialog({ open, onOpenChange, target }: Props) {
  const leaveGuild = useAppStore((s) => s.leaveGuild);
  const navigate = useNavigate();
  const [isLeaving, setIsLeaving] = useState(false);

  const handleLeave = async () => {
    if (!target) return;
    setIsLeaving(true);
    try {
      await leaveGuild(target.id);
      navigate({ to: "/g/@me" });
      toast.success(`Left ${target.name}`);
    } catch (e) {
      toast.error("Failed to leave guild", { description: String(e) });
    } finally {
      setIsLeaving(false);
      onOpenChange(false);
    }
  };

  return (
    <AlertDialog open={open} onOpenChange={onOpenChange}>
      <AlertDialogContent size="sm">
        <AlertDialogHeader>
          <AlertDialogTitle>Leave {target?.name}?</AlertDialogTitle>
          <AlertDialogDescription>
            You'll need an invite to rejoin. This cannot be undone.
          </AlertDialogDescription>
        </AlertDialogHeader>
        <AlertDialogFooter>
          <AlertDialogCancel disabled={isLeaving}>Cancel</AlertDialogCancel>
          <AlertDialogAction
            onClick={handleLeave}
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
  );
}
