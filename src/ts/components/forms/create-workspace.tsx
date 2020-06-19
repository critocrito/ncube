import {Form, Formik} from "formik";
import React from "react";
import * as Yup from "yup";

import Button from "../common/button";
import Input from "../common/input";

export interface CreateWorkspaceFormValues {
  name: string;
  description?: string;
  database: "sqlite";
  kind: "local";
}

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

interface CreateWorkspaceFormProps {
  onSubmit: (values: CreateWorkspaceFormValues) => void;
  onCancel: () => void;
  initialValues?: CreateWorkspaceFormValues;
}

const CreateWorkspaceForm = ({
  initialValues = defaultValues,
  onCancel,
  onSubmit,
}: CreateWorkspaceFormProps) => {
  return (
    <Formik
      initialValues={initialValues}
      validationSchema={validationSchema}
      onSubmit={onSubmit}
    >
      {(formik) => {
        const disableSubmit = !formik.isValid || formik.isSubmitting;

        return (
          <Form>
            <Input label="What is the name of this workspace?" name="name" />
            <Input
              label="If you like, you can add a short description."
              name="description"
            />

            <input name="database" type="hidden" value="sqlite" />
            <input name="kind" type="hidden" value="local" />

            <div className="flex justify-between ml-auto w-80">
              <Button
                type="reset"
                size="large"
                kind="secondary"
                onClick={onCancel}
              >
                Cancel
              </Button>

              <Button
                className="fr"
                type="submit"
                size="large"
                disabled={disableSubmit}
              >
                Create Workspace
              </Button>
            </div>
          </Form>
        );
      }}
    </Formik>
  );
};

export default CreateWorkspaceForm;
