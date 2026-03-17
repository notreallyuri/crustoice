import {
  Item,
  ItemActions,
  ItemContent,
  ItemMedia,
  ItemTitle
} from "@/components/ui/item";
import { ChevronRight, LucideIcon } from "lucide-react";

type SelectItemProps = {
  label: string;
  onClick: () => void;
  icon: LucideIcon;
};

export function SelectItem({ icon: Icon, onClick, label }: SelectItemProps) {
  return (
    <Item
      variant="outline"
      size="xs"
      onClick={onClick}
      className="hover:bg-muted/20 transition-colors cursor-pointer"
    >
      <ItemMedia className="size-8 bg-muted/25 border rounded-full  transition-colors">
        <Icon className="size-4" />
      </ItemMedia>
      <ItemContent>
        <ItemTitle className="select-none">{label}</ItemTitle>
      </ItemContent>
      <ItemActions>
        <ChevronRight className="size-4" />
      </ItemActions>
    </Item>
  );
}
