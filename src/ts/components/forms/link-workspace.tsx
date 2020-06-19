import {Form, Formik} from "formik";
import React from "react";
import * as Yup from "yup";

import Button from "../common/button";
import Input from "../common/input";

export interface LinkWorkspaceFormValues {
  workspace: string;
  name: string;
  description?: string;
  endpoint: string;
  email: string;
  otp: string;
  password: string;
  password_again: string;
  database: "http";
  kind: "remote";
}

export const validationSchema = Yup.object({
  workspace: Yup.string()
    .required("This workspace requires an identifier.")
    .max(150, "A workspace name cannot have more than 150 characters."),
  name: Yup.string()
    .required("Please provide a name for this workspace.")
    .max(150, "A workspace name cannot have more than 150 characters."),
  description: Yup.string(),
  endpoint: Yup.string()
    .url()
    .required("Please provide the endpoint for this remote workspace."),
  email: Yup.string()
    .email()
    .required("Your account must be linked to an Email address."),
  otp: Yup.string().required(
    "Provide your One-Time-Password to link to the remote workspace.",
  ),
  password: Yup.string()
    .required("Provide a password to reset the One-Time-Password")
    .min(15, "A password must be at least 15 characters long.")
    .max(250, "A password can be at least 250 characters long."),
  password_again: Yup.string().oneOf(
    [Yup.ref("password"), undefined],
    "Passwords must match",
  ),
  database: Yup.string().oneOf(["http"]).required(),
  kind: Yup.string().oneOf(["remote"]).required(),
});

interface LinkWorkspaceFormProps {
  onSubmit: (values: LinkWorkspaceFormValues) => void;
  onCancel: () => void;
  initialValues: LinkWorkspaceFormValues;
}

const LinkWorkspaceForm = ({
  initialValues,
  onSubmit,
  onCancel,
}: LinkWorkspaceFormProps) => {
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
            <Input
              label="The workspace identifier."
              name="workspace"
              disabled
            />

            <Input label="The workspace name." name="name" disabled />
            <Input
              label="A description of the workspace."
              name="description"
              disabled
            />

            <Input
              label="The endpoint of the workspace."
              name="endpoint"
              disabled
            />

            <Input
              label="The email of the account associated with the workspace."
              name="email"
              disabled
            />

            <Input
              label="The One-Time-Password to initialize the workspace."
              name="otp"
              disabled
            />

            <Input label="Please provide a new password." name="password" />

            <Input label="Repeat your new password." name="password_again" />

            <input name="database" type="hidden" value="http" />
            <input name="kind" type="hidden" value="remote" />

            <div className="flex justify-between ml-auto w-80">
              <Button
                type="reset"
                size="large"
                kind="secondary"
                onClick={onCancel}
              >
                Cancel
              </Button>

              <Button type="submit" size="large" disabled={disableSubmit}>
                Link Workspace
              </Button>
            </div>
          </Form>
        );
      }}
    </Formik>
  );
};

export default LinkWorkspaceForm;
