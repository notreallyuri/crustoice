import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter
} from "@/components/ui/dialog";
import {
  Field,
  FieldLabel,
  FieldDescription,
  FieldError,
  FieldGroup
} from "@/components/ui/field";
import {
  InputGroup,
  InputGroupAddon,
  InputGroupInput,
  InputGroupText
} from "@/components/ui/input-group";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue
} from "@/components/ui/select";
import { Button } from "@/components/ui/button";
import { useForm } from "@tanstack/react-form";
import {
  FileText,
  Hash,
  Layers,
  MessageSquare,
  Volume2,
  LayoutGrid
} from "lucide-react";
import z from "zod";
import { cn } from "@/lib/utils";
import { useAppStore } from "@/store/app-store";
import { toast } from "sonner";
import type { ChannelMode } from "@/types";

const MAX_NAME_LENGTH = 24;

const formSchema = z.object({
  name: z
    .string()
    .min(1, "Channel name is required.")
    .max(
      MAX_NAME_LENGTH,
      `Channel name must be at most ${MAX_NAME_LENGTH} characters.`
    ),
  kind: z.enum(["text", "voice", "docs", "canvas"]),
  mode: z.enum(["chat", "board", "threads"]),
  categoryId: z.string().nullable()
});

type ChannelKind = "text" | "voice" | "docs" | "canvas";

const KIND_OPTIONS: {
  value: ChannelKind;
  label: string;
  icon: React.ReactNode;
  description: string;
}[] = [
  {
    value: "text",
    label: "Text",
    icon: <Hash className="size-4" />,
    description: "Chat, boards or threads"
  },
  {
    value: "voice",
    label: "Voice",
    icon: <Volume2 className="size-4" />,
    description: "Voice communication"
  },
  {
    value: "docs",
    label: "Docs",
    icon: <FileText className="size-4" />,
    description: "Collaborative document"
  },
  {
    value: "canvas",
    label: "Canvas",
    icon: <Layers className="size-4" />,
    description: "Visual workspace"
  }
];

const MODE_OPTIONS: {
  value: ChannelMode;
  label: string;
  icon: React.ReactNode;
  description: string;
}[] = [
  {
    value: "chat",
    label: "Chat",
    icon: <MessageSquare className="size-4" />,
    description: "Classic linear chat"
  },
  {
    value: "board",
    label: "Board",
    icon: <LayoutGrid className="size-4" />,
    description: "Cards in a grid"
  },
  {
    value: "threads",
    label: "Threads",
    icon: <Hash className="size-4" />,
    description: "Threaded conversations"
  }
];

type Props = {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  goBack: () => void;
  categories: { id: string; name: string }[];
};

export function DialogCreateChannel({
  open,
  onOpenChange,
  categories,
  goBack
}: Props) {
  const createChannel = useAppStore((s) => s.createChannel);

  const form = useForm({
    defaultValues: {
      name: "",
      kind: "text" as ChannelKind,
      mode: "chat" as ChannelMode,
      categoryId: null as string | null
    },
    validators: { onSubmit: formSchema },
    onSubmit: async ({ value }) => {
      try {
        const base = {
          name: value.name,
          category_id: value.categoryId
        };

        const payload = (() => {
          switch (value.kind) {
            case "text":
              return { ...base, kind: "text" as const, mode: value.mode };
            case "voice":
              return { ...base, kind: "voice" as const };
            case "docs":
              return { ...base, kind: "docs" as const };
            case "canvas":
              return { ...base, kind: "canvas" as const };
          }
        })();

        await createChannel(payload);
        onOpenChange(false);
        form.reset();
      } catch (e) {
        toast.error("Failed to create channel.");
        console.error(e);
      }
    }
  });

  function handleBack() {
    form.reset();
    goBack();
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-sm">
        <DialogHeader>
          <DialogTitle>Create Channel</DialogTitle>
        </DialogHeader>

        <form
          onSubmit={(e) => {
            e.preventDefault();
            form.handleSubmit();
          }}
        >
          <FieldGroup>
            {/* Kind selector */}
            <form.Field
              name="kind"
              children={(field) => (
                <Field>
                  <FieldLabel>Channel Type</FieldLabel>
                  <div className="grid grid-cols-2 gap-2">
                    {KIND_OPTIONS.map((opt) => (
                      <button
                        key={opt.value}
                        type="button"
                        onClick={() => field.handleChange(opt.value)}
                        className={cn(
                          "flex flex-col items-start gap-1 rounded-md border px-3 py-2 text-sm transition-colors text-left",
                          field.state.value === opt.value
                            ? "border-primary bg-primary/10 text-primary-foreground"
                            : "border-border text-muted-foreground hover:bg-muted/20"
                        )}
                      >
                        <span className="flex items-center gap-2 font-medium">
                          {opt.icon}
                          {opt.label}
                        </span>
                        <span className="text-xs opacity-60">
                          {opt.description}
                        </span>
                      </button>
                    ))}
                  </div>
                </Field>
              )}
            />

            {/* Mode selector — only shown for text channels */}
            <form.Subscribe
              selector={(s) => s.values.kind}
              children={(kind) =>
                kind === "text" ? (
                  <form.Field
                    name="mode"
                    children={(field) => (
                      <Field>
                        <FieldLabel>Mode</FieldLabel>
                        <div className="grid grid-cols-3 gap-2">
                          {MODE_OPTIONS.map((opt) => (
                            <button
                              key={opt.value}
                              type="button"
                              onClick={() => field.handleChange(opt.value)}
                              className={cn(
                                "flex flex-col items-start gap-1 rounded-md border px-3 py-2 text-sm transition-colors text-left",
                                field.state.value === opt.value
                                  ? "border-primary bg-primary/10 text-primary-foreground"
                                  : "border-border text-muted-foreground hover:bg-muted/20"
                              )}
                            >
                              <span className="flex items-center gap-1.5 font-medium">
                                {opt.icon}
                                {opt.label}
                              </span>
                              <span className="text-xs opacity-60">
                                {opt.description}
                              </span>
                            </button>
                          ))}
                        </div>
                      </Field>
                    )}
                  />
                ) : null
              }
            />

            {/* Name */}
            <form.Field
              name="name"
              children={(field) => {
                const isInvalid =
                  field.state.meta.isTouched && !field.state.meta.isValid;
                const remaining = MAX_NAME_LENGTH - field.state.value.length;
                return (
                  <Field data-invalid={isInvalid}>
                    <FieldLabel htmlFor="channel-name">Channel Name</FieldLabel>
                    <InputGroup>
                      <InputGroupInput
                        id="channel-name"
                        placeholder="general"
                        value={field.state.value}
                        onBlur={field.handleBlur}
                        onChange={(e) => field.handleChange(e.target.value)}
                        aria-invalid={isInvalid}
                        maxLength={MAX_NAME_LENGTH}
                      />
                      <InputGroupAddon align="inline-end">
                        <InputGroupText
                          className={
                            remaining <= 5
                              ? "text-destructive"
                              : "text-muted-foreground"
                          }
                        >
                          {remaining}
                        </InputGroupText>
                      </InputGroupAddon>
                    </InputGroup>
                    {isInvalid && (
                      <FieldError errors={field.state.meta.errors} />
                    )}
                  </Field>
                );
              }}
            />

            {/* Category */}
            <form.Field
              name="categoryId"
              children={(field) => (
                <Field>
                  <FieldLabel>Category</FieldLabel>
                  <Select
                    value={field.state.value ?? ""}
                    onValueChange={(v) => field.handleChange(v || null)}
                  >
                    <SelectTrigger>
                      <SelectValue placeholder="No category" />
                    </SelectTrigger>
                    <SelectContent>
                      {categories.map((cat) => (
                        <SelectItem key={cat.id} value={cat.id}>
                          {cat.name}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                  <FieldDescription className="ml-2 text-xs">
                    Optional — leave empty for no category.
                  </FieldDescription>
                </Field>
              )}
            />
          </FieldGroup>

          <DialogFooter className="mt-4">
            <form.Subscribe
              selector={(s) => s.isSubmitting}
              children={(isSubmitting) => (
                <>
                  <Button
                    variant="outline"
                    type="button"
                    onClick={handleBack}
                    disabled={isSubmitting}
                  >
                    Cancel
                  </Button>
                  <Button type="submit" disabled={isSubmitting}>
                    {isSubmitting ? "Creating..." : "Create Channel"}
                  </Button>
                </>
              )}
            />
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}
