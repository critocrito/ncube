import {Form, Formik} from "formik";
import React from "react";

import BooleanSelect from "../components/boolean-select";
import {FormProps} from "../types";
import FormActions from "../components/form-actions";

type DeleteWorkspaceFormProps<
  DeleteWorkspaceFormValues
> = FormProps<DeleteWorkspaceFormValues>;

export interface DeleteWorkspaceFormValues {
  delete_location: boolean;
}

export const defaultValues: DeleteWorkspaceFormValues = {
  delete_location: false,
};

const DeleteWorkspaceForm = ({
  initialValues = defaultValues,
  onCancel,
  onSubmit,
}: DeleteWorkspaceFormProps<DeleteWorkspaceFormValues>) => {
  const formValues = {...defaultValues, ...initialValues};

  return (
    <Formik initialValues={formValues} onSubmit={onSubmit}>
      {({isValid, isSubmitting}) => {
        const disableSubmit = !isValid || isSubmitting;

        return (
          <Form>
            <BooleanSelect
              label="Delete workspace location?"
              name="delete_location"
              placeholder="no"
              isClearable={false}
            />

            <FormActions
              submitLabel="Remove Workspace"
              onCancel={onCancel}
              isDisabled={disableSubmit}
            />
          </Form>
        );
      }}
    </Formik>
  );
};

export default DeleteWorkspaceForm;
