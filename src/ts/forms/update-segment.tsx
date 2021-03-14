import {Form, Formik} from "formik";
import React from "react";
import * as Yup from "yup";

import Button from "../common/button";
import Input from "../common/input";
import {FormProps} from "../types";

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

            <div className="flex justify-between ml-auto w-80 pv3  pv2">
              <Button
                type="reset"
                size="large"
                kind="secondary"
                onClick={onCancel}
              >
                Cancel
              </Button>

              <Button
                className="fr"
                type="submit"
                size="large"
                disabled={disableSubmit}
              >
                Update Segment
              </Button>
            </div>
          </Form>
        );
      }}
    </Formik>
  );
};

export default UpdateSegmentForm;
