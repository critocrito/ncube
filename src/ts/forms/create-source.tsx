import {FieldArray, Form, Formik} from "formik";
import React, {useEffect, useState} from "react";
import * as Yup from "yup";

import FormActions from "../components/form-actions";
import Input from "../components/input";
import {listSourceTags} from "../lib/http";
import {sourceTags} from "../lib/validations";
import {FormProps, SourceTag} from "../types";
import SourceTagSelect from "../components/source-tag-select";
import SourceTypeSelect from "../components/source-type-select";

type CreateSourceFormProps<
  CreateSourceFormValues
> = FormProps<CreateSourceFormValues>;

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
  workspace,
}: CreateSourceFormProps<CreateSourceFormValues>) => {
  const [sourceTagsData, setSourceTagsData] = useState<SourceTag[]>([]);
  const formValues = {...defaultValues, ...initialValues};

  useEffect(() => {
    const fetchData = async () => {
      if (!workspace) return;
      const fetchedData = await listSourceTags(workspace.slug);
      setSourceTagsData(fetchedData);
    };
    fetchData();
  }, [workspace]);

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

            <SourceTypeSelect
              label="Select the source type"
              name="type"
              isClearable={false}
            />

            <FieldArray
              name="tags"
              render={() => {
                return (
                  <SourceTagSelect
                    options={sourceTagsData.map(({label, description}) => ({
                      label,
                      description,
                      value: label,
                    }))}
                    name="tags"
                    label="Source Tags"
                  />
                );
              }}
            />

            <FormActions
              submitLabel="Create Source"
              onCancel={onCancel}
              isDisabled={disableSubmit}
            />
          </Form>
        );
      }}
    </Formik>
  );
};

export default CreateSourceForm;
