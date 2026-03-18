import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle
} from "@/components/ui/dialog";
import { Field, FieldLabel } from "@/components/ui/field";
import {
  InputGroup,
  InputGroupAddon,
  InputGroupInput
} from "@/components/ui/input-group";
import { useForm } from "@tanstack/react-form";
import z from "zod";

const MAX_NAME_LENGTH = 24;

const formSchema = z.object({
  name: z
    .string()
    .min(1, "Channel name is required.")
    .max(
      MAX_NAME_LENGTH,
      `Channel name must be at most ${MAX_NAME_LENGTH} characters.`
    )
});

type Props = {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  goBack: () => void;
};

export function DialogCreateCategory({ open, onOpenChange, goBack }: Props) {
  const form = useForm({
    defaultValues: {
      name: ""
    },
    validators: {
      onSubmit: formSchema
    }
  });

  function handleBack() {
    form.reset();
    goBack();
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Create Category</DialogTitle>
        </DialogHeader>
        <form>
          <form.Field
            name="name"
            children={(field) => {
              return (
                <Field>
                  <FieldLabel htmlFor={field.name}></FieldLabel>
                  <InputGroup>
                    <InputGroupInput
                      id={field.name}
                      value={field.state.value}
                      onChange={(e) => field.setValue(e.target.value)}
                      maxLength={MAX_NAME_LENGTH}
                    />
                    <InputGroupAddon
                      align="inline-end"
                      className="h-full border-l pl-2"
                    >
                      {MAX_NAME_LENGTH - field.state.value.length}
                    </InputGroupAddon>
                  </InputGroup>
                </Field>
              );
            }}
          />
        </form>
        <DialogFooter>
          <Button variant="outline" onClick={handleBack}>
            Cancel
          </Button>
          <Button>Create</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
