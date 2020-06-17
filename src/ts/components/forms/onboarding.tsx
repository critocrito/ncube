import {Form, Formik} from "formik";
import React from "react";
import * as Yup from "yup";

import Button from "../base/button";
import Input from "../base/input";

interface OnboardingFormProps {
  onSubmit: (values: OnboardingFormValues) => void;
  initialValues?: OnboardingFormValues;
}

export interface OnboardingFormValues {
  workspace_root: string;
  name: string;
  email: string;
}

export const defaultValues: OnboardingFormValues = {
  workspace_root: "~/Ncube",
  name: "",
  email: "",
};

export const validationSchema = Yup.object({
  workspace_root: Yup.string().required("This field is required."),
  name: Yup.string(),
  email: Yup.string().email("Invalid email address."),
});

const OnboardingForm = ({
  initialValues = defaultValues,
  onSubmit,
}: OnboardingFormProps) => {
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
              label="Workspace Root Directory"
              name="workspace_root"
              placeholder="e.g. ~/Ncube"
            />
            <Input
              label="What is your name?"
              name="name"
              placeholder="e.g. Alice"
            />
            <Input
              label="What is your email?"
              name="email"
              type="email"
              placeholder="jane@example.org"
            />
            <div>
              <Button
                className="fr"
                type="submit"
                size="large"
                disabled={disableSubmit}
              >
                Continue
              </Button>
            </div>
          </Form>
        );
      }}
    </Formik>
  );
};

export default OnboardingForm;
