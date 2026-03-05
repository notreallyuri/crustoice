import { Field, FieldError, FieldLabel } from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import { useForm } from "@tanstack/react-form";
import { createFileRoute, Link, useNavigate } from "@tanstack/react-router";
import { Button } from "@/components/ui/button";
import { Eye, EyeClosed } from "lucide-react";
import { useAppStore } from "@/store/app-store";
import { toast } from "sonner";
import { useState } from "react";
import { z } from "zod";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";

export const Route = createFileRoute("/auth/login")({
  component: RouteComponent
});

function RouteComponent() {
  const login = useAppStore((s) => s.login);
  const navigate = useNavigate();

  const [show, setShow] = useState<boolean>(false);

  const form = useForm({
    defaultValues: {
      email: "",
      password: ""
    },
    validators: {
      onSubmit: z.object({
        email: z.email("Please enter a valid email address."),
        password: z.string().min(1, "Password is required")
      })
    },
    onSubmit: async ({ value }) => {
      try {
        await login(value);

        navigate({ to: "/g/@me" });
      } catch (e) {
        toast.error("Login failed.", { description: String(e) });
      }
    }
  });

  return (
    <div className="flex h-screen w-full items-center justify-center dark">
      <Card className="w-96">
        <CardHeader className="-mb-4">
          <CardTitle className="text-3xl font-bold">Login</CardTitle>
        </CardHeader>
        <CardContent>
          <form
            className="space-y-2"
            onSubmit={(e) => {
              e.preventDefault();
              e.stopPropagation();
              form.handleSubmit();
            }}
          >
            <form.Field
              name="email"
              children={(field) => {
                const isInvalid =
                  field.state.meta.isTouched && !field.state.meta.isValid;

                return (
                  <Field data-invalid={isInvalid}>
                    <FieldLabel htmlFor={field.name}>Email</FieldLabel>
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
                    <FieldLabel htmlFor={field.name}>Password</FieldLabel>
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
                        size="icon"
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
            <Button type="submit" className="w-full">
              Login
            </Button>
          </form>
          <p className="mt-2 text-center text-sm text-muted-foreground">
            Doesn't have an account yet?{" "}
            <Link to="/auth/register" className="text-primary hover:underline">
              Register
            </Link>
          </p>
        </CardContent>
      </Card>
    </div>
  );
}
