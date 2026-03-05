import { Button } from "./ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger
} from "./ui/dialog";
import { Input } from "./ui/input";
import { useForm } from "@tanstack/react-form";
import z from "zod";
import { Field, FieldDescription, FieldError, FieldLabel } from "./ui/field";
import { useAppStore } from "@/store/app-store";
import { Camera, X } from "lucide-react";
import { useRef, useState } from "react";
import { toast } from "sonner";
import { open as openDialog } from "@tauri-apps/plugin-dialog";
import { readFile } from "@tauri-apps/plugin-fs";

type Props = {
  isOpen: boolean;
  setIsOpen: (v: boolean) => void;
};

export function DialogCreateGuild({ isOpen, setIsOpen }: Props) {
  const createGuild = useAppStore((s) => s.createGuild);

  const fileInputRef = useRef<HTMLInputElement>(null);

  const [previewUrl, setPreviewUrl] = useState<string | null>(null);
  const [imagePath, setImagePath] = useState<string | null>(null);

  const form = useForm({
    defaultValues: {
      name: ""
    },
    validators: {
      onSubmit: z.object({
        name: z
          .string()
          .min(5, "Guild name must be at least 5 characters.")
          .max(24, "Guild titles must be at most 24 characters")
      })
    },
    onSubmit: async ({ value: payload }) => {
      try {
        await createGuild(payload, imagePath);
        setIsOpen(false);
        form.reset();
      } catch (e) {
        toast.error("Failed to create guild. Please try again.", {
          description: String(e)
        });
      }
    }
  });

  const handleReset = (open: boolean) => {
    if (!open) {
      form.reset();
      setPreviewUrl(null);
      setImagePath(null);
    }
  };

  const handleSelectImage = async () => {
    try {
      const selected = await openDialog({
        multiple: false,
        filters: [
          {
            name: "Image",
            extensions: ["png", "jpg", "jpeg", "gif", "webp"]
          }
        ]
      });

      if (selected && typeof selected === "string") {
        setImagePath(selected);
        const fileBytes = await readFile(selected);
        const blob = new Blob([fileBytes]);
        setPreviewUrl(URL.createObjectURL(blob));
      }
    } catch (err) {
      toast.error("Could not open file dialog");
    }
  };

  const clearImage = () => {
    if (previewUrl) URL.revokeObjectURL(previewUrl);
    setPreviewUrl(null);
    setImagePath(null);
    if (fileInputRef.current) fileInputRef.current.value = "";
  };

  return (
    <Dialog open={isOpen} onOpenChange={handleReset}>
      <DialogTrigger asChild></DialogTrigger>
      <DialogContent className="dark text-foreground">
        <DialogHeader>
          <DialogTitle>Create Guild</DialogTitle>
        </DialogHeader>
        <form
          className="flex flex-col"
          onSubmit={(e) => {
            e.preventDefault();
            e.stopPropagation();
            form.handleSubmit();
          }}
        >
          <div className="flex justify-center pt-2">
            <div className="relative">
              <button
                type="button"
                onClick={handleSelectImage}
                className="group relative flex size-24 cursor-pointer items-center justify-center border-2 border-dashed border-border bg-background transition-colors hover:border-primary hover:bg-white/5 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-offset-2 focus-visible:ring-offset-background"
              >
                {previewUrl ? (
                  <>
                    <img
                      src={previewUrl}
                      alt="Guild Icon Preview"
                      className="size-full object-cover"
                    />
                    <div className="absolute inset-0 flex items-center justify-center bg-black/50 opacity-0 transition-opacity group-hover:opacity-100">
                      <Camera className="size-8 text-border" />
                    </div>
                  </>
                ) : (
                  <>
                    <Camera className="size-8 text-muted-foreground transition-colors group-hover:text-primary" />
                    <div className="absolute -right-2 -top-2 flex size-6 items-center justify-center bg-primary text-primary-foreground shadow-sm group-hover:bg-primary/90">
                      <span className="text-xl font-bold leading-none">+</span>
                    </div>
                  </>
                )}
              </button>

              {previewUrl && (
                <button
                  type="button"
                  onClick={clearImage}
                  className="absolute -right-2 -top-2 flex size-6 items-center justify-center  bg-destructive text-destructive-foreground shadow-sm cursor-pointer hover:bg-destructive/60 transition-opacity"
                  title="Remove image"
                >
                  <X className="size-4" />
                </button>
              )}
            </div>
          </div>
          <form.Field
            name="name"
            children={(field) => {
              const isInvalid =
                field.state.meta.isTouched && !field.state.meta.isValid;

              return (
                <Field data-invalid={isInvalid}>
                  <FieldLabel htmlFor={field.name}>Name</FieldLabel>
                  <Input
                    id={field.name}
                    name={field.name}
                    value={field.state.value}
                    onBlur={field.handleBlur}
                    onChange={(e) => field.handleChange(e.target.value)}
                    aria-invalid={isInvalid}
                    placeholder="Ex: Giant Guild"
                    autoComplete="off"
                  />
                  <FieldDescription>
                    Provide a guild name (don't worry too much)
                  </FieldDescription>
                  {isInvalid && <FieldError errors={field.state.meta.errors} />}
                </Field>
              );
            }}
          />
          <Button className="mt-4 cursor-pointer ml-auto" type="submit">
            Create
          </Button>
        </form>
      </DialogContent>
    </Dialog>
  );
}
