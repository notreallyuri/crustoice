import { Calendar, Hash } from "lucide-react";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle
} from "@/components/ui/dialog";
import { SelectItem } from "./select-item";

type Props = {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onCreateChannel: () => void;
  onCreateEvent: () => void;
};

export function DialogCreateMenu({
  onOpenChange,
  open,
  onCreateChannel,
  onCreateEvent
}: Props) {
  function openNext(next: () => void) {
    onOpenChange(false);
    setTimeout(next, 150);
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-sm">
        <DialogHeader>
          <DialogTitle>Create</DialogTitle>
        </DialogHeader>
        <div className="flex flex-col gap-2">
          <SelectItem
            icon={Hash}
            label="Create channel"
            onClick={() => openNext(() => onCreateChannel())}
          />
          <SelectItem
            icon={Calendar}
            label="Create Event"
            onClick={() => openNext(() => onCreateEvent())}
          />
        </div>
      </DialogContent>
    </Dialog>
  );
}
