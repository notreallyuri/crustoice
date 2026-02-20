import { Plus } from "lucide-react";
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
import { useState } from "react";

export function DialogCreateGuild() {
  const [isOpen, setIsOpen] = useState(false);
  const createGuild = useAppStore((s) => s.createGuild);

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
    onSubmit: async ({ value }) => {
      try {
        await createGuild(value.name);
        setIsOpen(false);
        form.reset();
      } catch (e) {
        console.error("Failed to create guild:", e);
      }
    }
  });

  return (
    <Dialog open={isOpen} onOpenChange={setIsOpen}>
      <DialogTrigger asChild>
        <Button
          size="icon"
          variant="ghost"
          className="rounded-lg cursor-pointer group"
        >
          <Plus className="group-hover:opacity-75 transition-opacity" />
        </Button>
      </DialogTrigger>
      <DialogContent className="dark text-foreground">
        <DialogHeader>
          <DialogTitle>Create Guild</DialogTitle>
        </DialogHeader>
        <form
          onSubmit={(e) => {
            e.preventDefault();
            form.handleSubmit();
          }}
        >
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
          <Button className="mt-4 cursor-pointer" type="submit">
            Create
          </Button>
        </form>
      </DialogContent>
    </Dialog>
  );
}
