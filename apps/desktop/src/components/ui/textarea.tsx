import { cn } from "@/lib/utils";
import TextareaAutosize, {
  type TextareaAutosizeProps
} from "react-textarea-autosize";

function Textarea({ className, ...props }: TextareaAutosizeProps) {
  return (
    <TextareaAutosize
      minRows={1}
      maxRows={6}
      data-slot="textarea"
      className={cn(
        "border-input resize-none pr-5 placeholder:text-muted-foreground focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:bg-input/30 flex w-full rounded border bg-transparent px-3 py-2 text-base shadow-xs transition-[color,box-shadow] outline-none focus-visible:ring-[3px] disabled:cursor-not-allowed disabled:opacity-50 md:text-sm",
        className
      )}
      {...props}
    />
  );
}

export { Textarea };
