import {Form, Formik} from "formik";
import React from "react";
import * as Yup from "yup";

import Input from "../components/input";
import {FormProps} from "../types";
import FormActions from "../components/form-actions";

export type UpdateSegmentFormValues = {
  title: string;
  query: string;
};

export const defaultValues: UpdateSegmentFormValues = {
  title: "",
  query: "",
};

export const validationSchema = Yup.object({
  title: Yup.string()
    .required("Please provide a name for this segment.")
    .max(150, "A segment name cannot have more than 150 characters."),
  query: Yup.string().required(),
});

type UpdateSegmentFormProps<T extends UpdateSegmentFormValues> = FormProps<T>;

const UpdateSegmentForm = ({
  initialValues = defaultValues,
  onCancel,
  onSubmit,
}: UpdateSegmentFormProps<UpdateSegmentFormValues>) => {
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
              label="Do you want to change the name of this segment as well?"
              name="title"
            />
            <Input
              label="The search query for this segment."
              name="query"
              disabled
            />

            <FormActions
              submitLabel="Update Segment"
              onCancel={onCancel}
              isDisabled={disableSubmit}
            />
          </Form>
        );
      }}
    </Formik>
  );
};

export default UpdateSegmentForm;
