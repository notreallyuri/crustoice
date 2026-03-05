import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Field, FieldError, FieldLabel } from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import { useAppStore } from "@/store/app-store";
import { useForm } from "@tanstack/react-form";
import { createFileRoute, Link, useNavigate } from "@tanstack/react-router";
import { Camera, Eye, EyeClosed, X } from "lucide-react";
import { useState, useEffect } from "react";
import { toast } from "sonner";
import { open as openDialog } from "@tauri-apps/plugin-dialog";
import z from "zod";
import { readFile } from "@tauri-apps/plugin-fs";

export const Route = createFileRoute("/auth/register")({
  component: RouteComponent
});

function RouteComponent() {
  const navigate = useNavigate();

  const [previewUrl, setPreviewUrl] = useState<string | null>(null);
  const [imagePath, setImagePath] = useState<string | null>(null);
  const [showPassword, setShowPassword] = useState<boolean>(false);

  const register = useAppStore((s) => s.register);
  const form = useForm({
    defaultValues: {
      email: "",
      username: "",
      password: "",
      display_name: ""
    },
    validators: {
      onSubmit: z.object({
        email: z.email("Please enter a valid email address."),
        username: z.string().min(1, "Username is required"),
        password: z.string().min(1, "Password is required"),
        display_name: z.string()
      })
    },
    onSubmit: async ({ value }) => {
      try {
        await register(value, imagePath);

        navigate({ to: "/g/@me" });
      } catch (e) {
        toast.error("Registration failed.", { description: String(e) });
      }
    }
  });

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
  };

  useEffect(() => {
    console.log("Preview URL changed:", previewUrl);
    console.log("Current image path:", imagePath);
  }, [previewUrl]);

  return (
    <div className="flex h-screen  w-full items-center justify-center dark">
      <Card className="w-96">
        <CardHeader className="-mb-4">
          <CardTitle className="text-3xl font-bold">Register</CardTitle>
        </CardHeader>
        <CardContent>
          <form
            onSubmit={(e) => {
              e.preventDefault();
              e.stopPropagation();
              form.handleSubmit();
            }}
            className="space-y-2"
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
              name="username"
              children={(field) => {
                const isInvalid =
                  field.state.meta.isTouched && !field.state.meta.isValid;

                return (
                  <Field>
                    <FieldLabel htmlFor={field.name}>
                      Username<span className="text-primary">*</span>
                    </FieldLabel>
                    <Input
                      id={field.name}
                      name={field.name}
                      value={field.state.value}
                      onBlur={field.handleBlur}
                      onChange={(e) => field.handleChange(e.target.value)}
                      aria-invalid={isInvalid}
                      autoComplete="off"
                    />
                    {isInvalid && (
                      <FieldError errors={field.state.meta.errors} />
                    )}
                  </Field>
                );
              }}
            />

            <form.Field
              name="email"
              children={(field) => {
                const isInvalid =
                  field.state.meta.isTouched && !field.state.meta.isValid;

                return (
                  <Field>
                    <FieldLabel htmlFor={field.name}>
                      Email<span className="text-primary">*</span>
                    </FieldLabel>
                    <Input
                      id={field.name}
                      name={field.name}
                      value={field.state.value}
                      onBlur={field.handleBlur}
                      onChange={(e) => field.handleChange(e.target.value)}
                      aria-invalid={isInvalid}
                      autoComplete="off"
                    />
                    {isInvalid && (
                      <FieldError errors={field.state.meta.errors} />
                    )}
                  </Field>
                );
              }}
            />

            <form.Field
              name="password"
              children={(field) => {
                const isInvalid =
                  field.state.meta.isTouched && !field.state.meta.isValid;

                return (
                  <Field data-invalid={isInvalid}>
                    <FieldLabel htmlFor={field.name}>
                      Password<span className="text-primary">*</span>
                    </FieldLabel>
                    <div className="relative">
                      <Input
                        id={field.name}
                        name={field.name}
                        value={field.state.value}
                        type={showPassword ? "text" : "password"}
                        onBlur={field.handleBlur}
                        onChange={(e) => field.handleChange(e.target.value)}
                        aria-invalid={isInvalid}
                        autoComplete="off"
                      />
                      <Button
                        variant="ghost"
                        type="button"
                        className="absolute right-0 top-1/2 -translate-y-1/2"
                        size="icon-xs"
                        onClick={() => setShowPassword(!showPassword)}
                        tabIndex={-1}
                      >
                        {showPassword ? <Eye /> : <EyeClosed />}
                      </Button>
                    </div>
                    {isInvalid && (
                      <FieldError errors={field.state.meta.errors} />
                    )}
                  </Field>
                );
              }}
            />

            <form.Field
              name="display_name"
              children={(field) => {
                const isInvalid =
                  field.state.meta.isTouched && !field.state.meta.isValid;

                return (
                  <Field>
                    <FieldLabel htmlFor={field.name}>Display Name</FieldLabel>
                    <Input
                      id={field.name}
                      name={field.name}
                      value={field.state.value}
                      onBlur={field.handleBlur}
                      onChange={(e) => field.handleChange(e.target.value)}
                      aria-invalid={isInvalid}
                      autoComplete="off"
                      placeholder={form.state.values.username}
                    />
                    {isInvalid && (
                      <FieldError errors={field.state.meta.errors} />
                    )}
                  </Field>
                );
              }}
            />
            <Button type="submit" className="w-full">
              Register
            </Button>
          </form>
          <p className="mt-2 text-center text-sm text-muted-foreground">
            Already have an account ?{" "}
            <Link to={"/auth/login"} className="text-primary hover:underline">
              Login
            </Link>
          </p>
        </CardContent>
      </Card>
    </div>
  );
}
