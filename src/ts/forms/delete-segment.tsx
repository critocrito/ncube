/* eslint react/no-array-index-key: off */
import {Form, Formik} from "formik";
import React from "react";

import Button from "../common/button";
import {FormProps} from "../types";

type DeleteSegmentFormProps<
  DeleteSegmentFormValues
> = FormProps<DeleteSegmentFormValues>;

// eslint-disable-next-line @typescript-eslint/no-empty-interface
export interface DeleteSegmentFormValues {}

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

export default DeleteSegmentForm;
