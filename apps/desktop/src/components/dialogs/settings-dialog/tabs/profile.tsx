import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { Button } from "@/components/ui/button";
import { Label } from "@/components/ui/label";
import { useCurrentUser } from "@/hooks/use-current-user";
import {
  InputGroup,
  InputGroupInput,
  InputGroupText,
  InputGroupAddon,
  InputGroupTextarea
} from "@/components/ui/input-group";
import { useState } from "react";
import { BrushCleaning, Camera, Trash2, Save } from "lucide-react";
import { useAppStore } from "@/store/app-store";
import z from "zod";
import { useForm } from "@tanstack/react-form";
import { toast } from "sonner";
import { DialogCropper } from "../../dialog-cropper";
import { Field, FieldError, FieldLabel } from "@/components/ui/field";
import { useImageSelection } from "@/hooks/use-image-selection";
import { cn } from "@/lib/utils";
import { CropResult } from "@/components/kibo-ui/image-crop";

const MAX_BIO_LENGTH = 250;
const MAX_NAME_LENGTH = 32;

export function ProfileSettings() {
  const {
    previewUrl,
    handleSelectImage,
    isSelecting,
    clearSelection,
    originalPath
  } = useImageSelection();
  const updateProfile = useAppStore((s) => s.updateProfile);
  const user = useCurrentUser();

  const [isUploadingAvatar, setIsUploadingAvatar] = useState(false);

  const form = useForm({
    defaultValues: {
      display_name: user.profile.display_name || "",
      bio: user.profile.bio || ""
    },
    validators: {
      onChange: z.object({
        display_name: z
          .string()
          .min(2, "Display name must be at least 2 characters.")
          .max(MAX_NAME_LENGTH, "Display name is too long."),
        bio: z.string().max(MAX_BIO_LENGTH, "Bio is too long.")
      })
    },
    onSubmit: async ({ value }) => {
      if (
        value.display_name === user.profile.display_name &&
        value.bio === (user.profile.bio || "")
      )
        return;

      try {
        await updateProfile({
          display_name:
            value.display_name !== user.profile.display_name
              ? value.display_name
              : undefined,
          bio: value.bio !== user.profile.bio ? value.bio : undefined
        });
        toast.success("Profile updated");
      } catch (e) {
        toast.error("Failed to update profile.", { description: String(e) });
        form.reset();
      }
    }
  });

  async function handleCropSuccess(cropMath: CropResult) {
    setIsUploadingAvatar(true);
    if (!originalPath) {
      setIsUploadingAvatar(false);
      return;
    }
    try {
      await updateProfile({ avatar_url: originalPath }, cropMath);
      toast.success("Avatar updated successfully");
    } catch (e) {
      toast.error("Failed to upload avatar", { description: String(e) });
    } finally {
      setIsUploadingAvatar(false);
      clearSelection();
    }
  }

  async function handleRemoveAvatar() {
    try {
      await updateProfile({ avatar_url: "" });
      toast.success("Avatar removed");
    } catch (e) {
      toast.error("Failed to remove avatar", {
        description: String(e)
      });
    }
  }

  return (
    <div className="w-full max-w-lg space-y-6 pb-10">
      {/* Avatar banner section */}
      <div className="relative rounded-lg border overflow-hidden">
        {/* Banner background */}
        <div className="h-24 bg-linear-to-br from-primary/30 via-primary/10 to-transparent border-b border-border" />

        {/* Avatar overlapping the banner */}
        <div className="px-4 pb-4">
          <div className="flex items-end justify-between -mt-10">
            <div className="relative group">
              <Avatar className="size-20 border-4 border-background ring-2 ring-border">
                <AvatarImage src={user.profile.avatar_url || undefined} />
                <AvatarFallback className="text-2xl bg-primary">
                  {user.profile.display_name.charAt(0)}
                </AvatarFallback>
              </Avatar>
              <button
                type="button"
                onClick={handleSelectImage}
                disabled={isSelecting || isUploadingAvatar}
                className="absolute inset-0 rounded-full flex items-center justify-center bg-black/60 opacity-0 group-hover:opacity-100 transition-opacity cursor-pointer"
              >
                <Camera className="size-6 text-white" />
              </button>
              {isUploadingAvatar && (
                <div className="absolute inset-0 rounded-full flex items-center justify-center bg-black/60">
                  <div className="size-5 border-2 border-white border-t-transparent rounded-full animate-spin" />
                </div>
              )}
            </div>

            <div className="flex gap-2 mb-1">
              {user.profile.avatar_url && (
                <Button
                  variant="ghost"
                  size="sm"
                  onClick={handleRemoveAvatar}
                  className="text-xs text-destructive hover:text-destructive hover:bg-destructive/10"
                >
                  <Trash2 className="size-3 mr-1" />
                  Remove
                </Button>
              )}
            </div>
          </div>

          <div className="mt-2">
            <p className="font-semibold text-foreground">
              {user.profile.display_name}
            </p>
            <p className="text-xs text-muted-foreground">
              @{user.profile.username}
            </p>
          </div>
        </div>
      </div>

      <div className="border-t border-border" />

      <form
        onSubmit={(e) => {
          e.preventDefault();
          e.stopPropagation();
          form.handleSubmit();
        }}
        className="space-y-5"
      >
        <form.Field
          name="display_name"
          children={(field) => {
            const isInvalid =
              field.state.meta.isTouched && !field.state.meta.isValid;
            return (
              <Field>
                <FieldLabel
                  htmlFor={field.name}
                  className="text-xs font-bold uppercase tracking-wider text-muted-foreground"
                >
                  Display Name
                </FieldLabel>
                <InputGroup className={cn(isInvalid && "border-destructive")}>
                  <InputGroupAddon align="inline-start">
                    <InputGroupText className="text-muted-foreground">
                      @
                    </InputGroupText>
                  </InputGroupAddon>
                  <InputGroupInput
                    id={field.name}
                    value={field.state.value}
                    onBlur={field.handleBlur}
                    onChange={(e) => field.handleChange(e.target.value)}
                    placeholder="Your display name..."
                    maxLength={MAX_NAME_LENGTH}
                  />
                  <InputGroupAddon
                    align="inline-end"
                    className="border-l px-2 flex items-center"
                  >
                    <span
                      className={cn(
                        "text-xs tabular-nums",
                        MAX_NAME_LENGTH - field.state.value.length <= 5
                          ? "text-destructive"
                          : "text-muted-foreground"
                      )}
                    >
                      {MAX_NAME_LENGTH - field.state.value.length}
                    </span>
                  </InputGroupAddon>
                </InputGroup>
                {isInvalid && <FieldError errors={field.state.meta.errors} />}
                <p className="text-xs text-muted-foreground">
                  Spaces and special characters are allowed.
                </p>
              </Field>
            );
          }}
        />

        <form.Field
          name="bio"
          children={(field) => {
            const remaining = MAX_BIO_LENGTH - field.state.value.length;
            return (
              <Field>
                <div className="flex items-center justify-between">
                  <Label
                    htmlFor={field.name}
                    className="text-xs font-bold uppercase tracking-wider text-muted-foreground"
                  >
                    Bio
                  </Label>
                </div>
                <InputGroup>
                  <InputGroupTextarea
                    placeholder="Tell people a little about yourself..."
                    id={field.name}
                    value={field.state.value}
                    onChange={(e) => field.handleChange(e.target.value)}
                    maxLength={MAX_BIO_LENGTH}
                    className="min-h-24 resize-none"
                  />
                  <InputGroupAddon
                    className="border-t justify-between py-1"
                    align="block-end"
                  >
                    <Button
                      type="button"
                      variant="ghost"
                      size="xs"
                      onClick={() => field.handleChange("")}
                      disabled={!field.state.value}
                    >
                      <BrushCleaning className="size-3 mr-1" />
                      Clear
                    </Button>
                    <span
                      className={cn(
                        "text-xs tabular-nums pr-1",
                        remaining <= 0
                          ? "text-destructive"
                          : remaining <= 30
                            ? "text-yellow-500"
                            : "text-muted-foreground"
                      )}
                    >
                      {remaining} left
                    </span>
                  </InputGroupAddon>
                </InputGroup>
              </Field>
            );
          }}
        />

        <div className="flex justify-end pt-2">
          <Button
            type="submit"
            disabled={!form.state.canSubmit || form.state.isSubmitting}
            className="gap-2"
          >
            {form.state.isSubmitting ? (
              <div className="size-4 border-2 border-current border-t-transparent rounded-full animate-spin" />
            ) : (
              <Save className="size-4" />
            )}
            Save Changes
          </Button>
        </div>
      </form>

      <DialogCropper
        previewUrl={previewUrl}
        onClose={clearSelection}
        onSuccess={handleCropSuccess}
      />
    </div>
  );
}
