import {Form, Formik} from "formik";
import React from "react";

import FormActions from "../components/form-actions";
import {FormProps} from "../types";

type DeleteSourceFormProps<
  DeleteSourceFormValues
> = FormProps<DeleteSourceFormValues>;

export type DeleteSourceFormValues = Record<string, unknown>;

export const defaultValues: DeleteSourceFormValues = {};

const DeleteSourceForm = ({
  initialValues = defaultValues,
  onCancel,
  onSubmit,
}: DeleteSourceFormProps<DeleteSourceFormValues>) => {
  const formValues = {...defaultValues, ...initialValues};

  return (
    <Formik initialValues={formValues} onSubmit={onSubmit}>
      {({isValid, isSubmitting}) => {
        const disableSubmit = !isValid || isSubmitting;

        return (
          <Form>
            <FormActions
              submitLabel="Delete Source"
              onCancel={onCancel}
              isDisabled={disableSubmit}
            />
          </Form>
        );
      }}
    </Formik>
  );
};

export default DeleteSourceForm;
