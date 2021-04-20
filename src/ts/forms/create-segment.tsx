import {Form, Formik} from "formik";
import React from "react";
import * as Yup from "yup";

import FormActions from "../components/form-actions";
import Input from "../components/input";
import {FormProps} from "../types";

export type CreateSegmentFormValues = {
  title: string;
  query: string;
};

export const defaultValues: CreateSegmentFormValues = {
  title: "",
  query: "",
};

export const validationSchema = Yup.object({
  title: Yup.string()
    .required("Please provide a name for this segment.")
    .max(150, "A segment name cannot have more than 150 characters."),
  query: Yup.string().required(),
});

type CreateSegmentFormProps<T extends CreateSegmentFormValues> = FormProps<T>;

const CreateSegmentForm = ({
  initialValues = defaultValues,
  onCancel,
  onSubmit,
}: CreateSegmentFormProps<CreateSegmentFormValues>) => {
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
            <Input label="What is the name of this segment?" name="title" />
            <Input
              label="The search query for this segment."
              name="query"
              disabled
            />

            <FormActions
              submitLabel="Create Segment"
              onCancel={onCancel}
              isDisabled={disableSubmit}
            />
          </Form>
        );
      }}
    </Formik>
  );
};

export default CreateSegmentForm;
