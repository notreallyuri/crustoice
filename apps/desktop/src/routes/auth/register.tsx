import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Field, FieldError, FieldLabel } from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import { useAppStore } from "@/store/app-store";
import { useForm } from "@tanstack/react-form";
import { createFileRoute, Link } from "@tanstack/react-router";
import { Eye, EyeClosed } from "lucide-react";
import { useState } from "react";
import { toast } from "sonner";
import z from "zod";

export const Route = createFileRoute("/auth/register")({
  component: RouteComponent
});

function RouteComponent() {
  const [show, setShow] = useState<boolean>(false);
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
        await register(
          value.email,
          value.username,
          value.password,
          value.display_name
        );
      } catch (e) {
        toast.error("Registration failed.", { description: String(e) });
      }
    }
  });

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
                        type={show ? "text" : "password"}
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
                        onClick={() => setShow(!show)}
                        tabIndex={-1}
                      >
                        {show ? <Eye /> : <EyeClosed />}
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
