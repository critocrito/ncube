import {Form, Formik} from "formik";
import React from "react";
import * as Yup from "yup";

import Button from "../common/button";
import Input from "../common/input";
import {FormProps} from "../types";

type CreateSourceFormProps<CreateSourceFormValues> = FormProps<
  CreateSourceFormValues
>;

export interface CreateSourceFormValues {
  type: string;
  term: string;
}

export const defaultValues: CreateSourceFormValues = {
  type: "",
  term: "",
};

export const validationSchema = Yup.object({
  type: Yup.string().required("This field is required."),
  term: Yup.string().required("This field is required."),
});

const CreateSourceForm = ({
  initialValues = defaultValues,
  onCancel,
  onSubmit,
}: CreateSourceFormProps<CreateSourceFormValues>) => {
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
            <Input label="Type" name="type" placeholder="e.g. youtube_video" />

            <Input
              label="Term"
              name="term"
              placeholder="e.g. http://youtube.com/watch?v=abcdef"
            />

            <div className="flex justify-between ml-auto w-80">
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
                Create Source
              </Button>
            </div>
          </Form>
        );
      }}
    </Formik>
  );
};

export default CreateSourceForm;
