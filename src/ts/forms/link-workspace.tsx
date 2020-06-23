import {Form, Formik} from "formik";
import React from "react";
import * as Yup from "yup";

import Button from "../common/button";
import Input from "../common/input";
import {ConnectionDetails, FormProps} from "../types";
import * as v from "../validations";

export type LinkWorkspaceFormValues = ConnectionDetails & {
  password: string;
  password_again: string;
  database: "http";
  kind: "remote";
};

export const defaultValues: LinkWorkspaceFormValues = {
  workspace: "",
  name: "",
  description: "",
  endpoint: "",
  email: "",
  otp: "",
  password: "",
  password_again: "",
  created_at: new Date().toISOString(),
  updated_at: new Date().toISOString(),
  database: "http",
  kind: "remote",
};

export const validationSchema = Yup.object({
  workspace: v.workspaceLabel,
  name: v.workspaceName,
  description: v.workspaceDescription,
  endpoint: v.workspaceEndpoint,
  email: v.email,
  otp: v.otp,
  password: v.password,
  password_again: Yup.string().oneOf(
    [Yup.ref("password"), undefined],
    "Passwords must match",
  ),
  database: Yup.string().oneOf(["http"]).required(),
  kind: Yup.string().oneOf(["remote"]).required(),
});

type LinkWorkspaceFormProps<LinkWorkspaceFormValues> = FormProps<
  LinkWorkspaceFormValues
>;

const LinkWorkspaceForm = ({
  initialValues = defaultValues,
  onSubmit,
  onCancel,
}: LinkWorkspaceFormProps<LinkWorkspaceFormValues>) => {
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
