import {Form, Formik} from "formik";
import React from "react";

import Button from "../components/button";
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
            <div className="flex justify-between ml-auto w-40 mt3">
              <Button type="reset" kind="secondary" onClick={onCancel}>
                Cancel
              </Button>
              <Button className="ml2" type="submit" disabled={disableSubmit}>
                Remove
              </Button>
            </div>
          </Form>
        );
      }}
    </Formik>
  );
};

export default DeleteSourceForm;
