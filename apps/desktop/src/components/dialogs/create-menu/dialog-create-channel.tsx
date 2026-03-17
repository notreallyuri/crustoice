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
import { Hash, Volume2 } from "lucide-react";
import z from "zod";
import { cn } from "@/lib/utils";
import { useAppStore } from "@/store/app-store";
import { toast } from "sonner";

const MAX_NAME_LENGTH = 24;

const formSchema = z.object({
  name: z
    .string()
    .min(1, "Channel name is required.")
    .max(
      MAX_NAME_LENGTH,
      `Channel name must be at most ${MAX_NAME_LENGTH} characters.`
    ),
  type: z.enum(["text", "voice"]),
  categoryId: z.string().nullable()
});

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
      type: "text" as "text" | "voice",
      categoryId: null as string | null
    },
    validators: {
      onSubmit: formSchema
    },
    onSubmit: async ({ value }) => {
      try {
        const payload =
          value.type === "text"
            ? {
                kind: "text" as const,
                name: value.name,
                category_id: value.categoryId,
                mode: "chat" as const
              }
            : {
                kind: "voice" as const,
                name: value.name,
                category_id: value.categoryId
              };

        await createChannel(payload);
        onOpenChange(false);
        form.reset();
      } catch (e) {
        toast.error("Failed to create channel.");
        console.error(e);
      }
    }
  });

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
            <form.Field
              name="type"
              children={(field) => (
                <Field>
                  <FieldLabel>Channel Type</FieldLabel>
                  <div className="grid grid-cols-2 gap-2">
                    {(["text", "voice"] as const).map((t) => (
                      <button
                        key={t}
                        type="button"
                        onClick={() => field.handleChange(t)}
                        className={cn(
                          "flex items-center gap-2 rounded-md border px-3 py-2 text-sm transition-colors",
                          field.state.value === t
                            ? "border-primary bg-primary/10 text-primary-foreground"
                            : "border-border text-muted-foreground hover:bg-muted/20"
                        )}
                      >
                        {t === "text" ? (
                          <Hash className="size-4" />
                        ) : (
                          <Volume2 className="size-4" />
                        )}
                        {t.charAt(0).toUpperCase() + t.slice(1)}
                      </button>
                    ))}
                  </div>
                </Field>
              )}
            />

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
                    onClick={goBack}
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
