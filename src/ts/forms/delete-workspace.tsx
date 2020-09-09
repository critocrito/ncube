/* eslint react/no-array-index-key: off */
import {Form, Formik} from "formik";
import React from "react";

import BooleanSelect from "../common/boolean-select";
import Button from "../common/button";
import {FormProps} from "../types";

type DeleteWorkspaceFormProps<DeleteWorkspaceFormValues> = FormProps<
  DeleteWorkspaceFormValues
>;

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
              description="This will not only remove the workspace configuration, but any downloaded media and the database of this workspace as well."
              placeholder="no"
              isClearable={false}
            />

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

export default DeleteWorkspaceForm;
