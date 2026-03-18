import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { useForm } from "@tanstack/react-form";
import z from "zod";
import {
  Field,
  FieldDescription,
  FieldError,
  FieldLabel
} from "@/components/ui/field";
import { useAppStore } from "@/store/app-store";
import { Camera, X } from "lucide-react";
import { useEffect, useState } from "react";
import { toast } from "sonner";
import { DialogCropper } from "@/components/dialogs/dialog-cropper";
import { useImageSelection } from "@/hooks/use-image-selection";
import { CropResult } from "@/components/kibo-ui/image-crop";

type Props = {
  isOpen: boolean;
  setIsOpen: (v: boolean) => void;
};

export function DialogCreateGuild({ isOpen, setIsOpen }: Props) {
  const { previewUrl, handleSelectImage, clearSelection, originalPath } =
    useImageSelection();

  const createGuild = useAppStore((s) => s.createGuild);

  const [isCropperOpen, setIsCropperOpen] = useState(false);
  const [cropMath, setCropMath] = useState<CropResult | undefined>();

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
        await createGuild(payload, originalPath, cropMath);

        setIsOpen(false);
        handleReset();
      } catch (e) {
        toast.error("Failed to create guild. Please try again.", {
          description: String(e)
        });
      }
    }
  });

  useEffect(() => {
    if (previewUrl) {
      setIsCropperOpen(true);
    }
  }, [previewUrl]);

  function handleCropSuccess(math: CropResult) {
    setCropMath(math);
    setIsCropperOpen(false);
  }

  function handleRemoveImage() {
    clearSelection();
    setCropMath(undefined);
  }

  function handleReset() {
    form.reset();
    clearSelection();
    setCropMath(undefined);
  }

  return (
    <>
      <Dialog
        open={isOpen}
        onOpenChange={(v) => {
          if (!v) {
            handleReset();
          }
          setIsOpen(v);
        }}
      >
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
                  className="group rounded-full relative flex size-24 cursor-pointer items-center justify-center border-2 border-dashed border-border bg-background transition-colors hover:border-primary hover:bg-white/5 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-offset-2 focus-visible:ring-offset-background"
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
                      <div className="absolute rounded-full -right-0.5 -top-0.5  flex size-6 items-center justify-center bg-primary text-primary-foreground shadow-sm group-hover:bg-primary/90">
                        <span className="text-xl font-bold leading-none">
                          +
                        </span>
                      </div>
                    </>
                  )}
                </button>

                {previewUrl && (
                  <button
                    type="button"
                    onClick={handleRemoveImage}
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
                    {isInvalid && (
                      <FieldError errors={field.state.meta.errors} />
                    )}
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

      <DialogCropper
        isOpen={isCropperOpen}
        previewUrl={previewUrl}
        onClose={() => setIsCropperOpen(false)}
        onSuccess={handleCropSuccess}
      />
    </>
  );
}
