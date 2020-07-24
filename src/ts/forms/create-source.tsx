/* eslint react/no-array-index-key: off */
import {FieldArray, Form, Formik} from "formik";
import React, {useEffect, useState} from "react";
import * as Yup from "yup";

import Button from "../common/button";
import Input from "../common/input";
import {listSourceTags} from "../http";
import {FormProps, SourceTag} from "../types";
import {sourceTags} from "../validations";
import SourceTagMultiSelect from "./source-tag-multi-select";

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

            <Input label="Type" name="type" placeholder="e.g. youtube_video" />

            <FieldArray
              name="tags"
              render={(helpers) => {
                return (
                  <SourceTagMultiSelect
                    className="mt3"
                    data={sourceTagsData}
                    onRemove={(tag: SourceTag) => {
                      const index = values.tags.findIndex(
                        (t) => t.label === tag.label,
                      );
                      helpers.remove(index);
                    }}
                    onAdd={(tag: SourceTag) => {
                      helpers.push(tag);
                    }}
                  />
                );
              }}
            />

            <div className="flex justify-between ml-auto w-80 pv3 ">
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
