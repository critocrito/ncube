/* eslint react/no-array-index-key: off */
import {FieldArray, Form, Formik} from "formik";
import React from "react";
import * as Yup from "yup";

import Button from "../common/button";
import Input from "../common/input";
import {FormProps, SourceTag} from "../types";
import {sourceTags} from "../validations";

type CreateSourceFormProps<CreateSourceFormValues> = FormProps<
  CreateSourceFormValues
>;

export interface CreateSourceFormValues {
  type: string;
  term: string;
  tags: SourceTag[];
}

export const defaultValues: CreateSourceFormValues = {
  type: "",
  term: "",
  tags: [],
};

export const validationSchema = Yup.object({
  type: Yup.string().required("This field is required."),
  term: Yup.string().required("This field is required."),
  tags: sourceTags,
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
      {({isValid, isSubmitting, values}) => {
        const disableSubmit = !isValid || isSubmitting;

        return (
          <Form>
            <Input
              label="Term"
              name="term"
              placeholder="e.g. http://youtube.com/watch?v=abcdef"
            />

            <Input label="Type" name="type" placeholder="e.g. youtube_video" />

            <FieldArray
              name="tags"
              render={(helpers) => {
                return (
                  <div className="flex flex-column">
                    <div>
                      <Button
                        size="normal"
                        onClick={() => helpers.push({name: "", value: ""})}
                      >
                        Add Tag
                      </Button>
                    </div>

                    {values.tags.map((_tag, index) => {
                      return (
                        <div
                          key={`tag-${index}`}
                          className="flex justify-between items-start"
                        >
                          <Input label="Name" name={`tags[${index}].name`} />
                          <Input label="Value" name={`tags[${index}].value`} />

                          <div style={{marginTop: "2.8rem"}}>
                            <Button
                              kind="secondary"
                              onClick={() => helpers.remove(index)}
                            >
                              Remove
                            </Button>
                          </div>
                        </div>
                      );
                    })}
                  </div>
                );
              }}
            />

            <div className="flex justify-between ml-auto w-80 pv2">
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
