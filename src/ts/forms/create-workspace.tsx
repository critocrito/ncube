import {Form, Formik} from "formik";
import React from "react";
import * as Yup from "yup";

import Input from "../components/input";
import FormActions from "../components/form-actions";
import {FormProps} from "../types";

export type CreateWorkspaceFormValues = {
  name: string;
  description?: string;
  database: "sqlite";
  kind: "local";
};

export const defaultValues: CreateWorkspaceFormValues = {
  name: "",
  description: "",
  database: "sqlite",
  kind: "local",
};

export const validationSchema = Yup.object({
  name: Yup.string()
    .required("Please provide a name for this workspace.")
    .max(150, "A workspace name cannot have more than 150 characters."),
  description: Yup.string(),
  database: Yup.string().oneOf(["sqlite"]).required(),
  kind: Yup.string().oneOf(["local"]).required(),
});

type CreateWorkspaceFormProps<
  T extends CreateWorkspaceFormValues
> = FormProps<T>;

const CreateWorkspaceForm = ({
  initialValues = defaultValues,
  onCancel,
  onSubmit,
}: CreateWorkspaceFormProps<CreateWorkspaceFormValues>) => {
  const formValues = {...defaultValues, ...initialValues};

  return (
    <Formik
      initialValues={formValues}
      validationSchema={validationSchema}
      onSubmit={onSubmit}
    >
      {(formik) => {
        const disableSubmit = !formik.isValid || formik.isSubmitting;

        return (
          <Form className="space-y-4">
            <Input label="What is the name of this workspace?" name="name" />
            <Input
              label="If you like, you can add a short description."
              name="description"
            />

            <input name="database" type="hidden" value="sqlite" />
            <input name="kind" type="hidden" value="local" />

            <FormActions
              submitLabel="Create Workspace"
              onCancel={onCancel}
              isDisabled={disableSubmit}
            />
          </Form>
        );
      }}
    </Formik>
  );
};

export default CreateWorkspaceForm;
