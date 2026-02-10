import { Field, FieldDescription, FieldLabel } from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import type { ChangeEvent } from "react";

export default function FileInput({
  onChangeInput,
}: {
  onChangeInput?: (eve: ChangeEvent<HTMLInputElement>) => void;
}) {
  return (
    <Field className="w-72">
      <FieldLabel htmlFor="picture">File</FieldLabel>
      <Input id="picture" type="file" onChange={onChangeInput} />
      <FieldDescription>Select file to upload</FieldDescription>
    </Field>
  );
}
