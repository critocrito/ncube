import {Form, Formik} from "formik";
import React from "react";

import FormActions from "../components/form-actions";
import {FormProps} from "../types";

type DeleteSegmentFormProps<
  DeleteSegmentFormValues
> = FormProps<DeleteSegmentFormValues>;

export type DeleteSegmentFormValues = Record<string, unknown>;

export const defaultValues: DeleteSegmentFormValues = {};

const DeleteSegmentForm = ({
  initialValues = defaultValues,
  onCancel,
  onSubmit,
}: DeleteSegmentFormProps<DeleteSegmentFormValues>) => {
  const formValues = {...defaultValues, ...initialValues};

  return (
    <Formik initialValues={formValues} onSubmit={onSubmit}>
      {({isValid, isSubmitting}) => {
        const disableSubmit = !isValid || isSubmitting;

        return (
          <Form>
            <FormActions
              submitLabel="Delete Segment"
              onCancel={onCancel}
              isDisabled={disableSubmit}
            />
          </Form>
        );
      }}
    </Formik>
  );
};

export default DeleteSegmentForm;
