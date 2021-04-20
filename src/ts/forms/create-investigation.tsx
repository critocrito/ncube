import {Form, Formik} from "formik";
import React, {useEffect, useState} from "react";
import * as Yup from "yup";

import DescriptionSelect from "../components/description-select";
import FormActions from "../components/form-actions";
import Input from "../components/input";
import Textarea from "../components/text-area";
import {listMethodologies} from "../lib/http";
import {FormProps, Methodology} from "../types";

type CreateInvestigationFormProps<
  CreateInvestigationFormValues
> = FormProps<CreateInvestigationFormValues>;

export interface CreateInvestigationFormValues {
  title: string;
  description: string;
  methodology: string;
}

export const defaultValues: CreateInvestigationFormValues = {
  title: "",
  description: "",
  methodology: "",
};

export const validationSchema = Yup.object({
  title: Yup.string().required("This field is required."),
  description: Yup.string(),
  methodology: Yup.string().required("This field is required."),
});

const CreateInvestigationForm = ({
  initialValues = defaultValues,
  onCancel,
  onSubmit,
  workspace,
}: CreateInvestigationFormProps<CreateInvestigationFormValues>) => {
  const [methodologiesData, setMethodologiesData] = useState<Methodology[]>([]);
  const formValues = {...defaultValues, ...initialValues};

  useEffect(() => {
    const fetchData = async () => {
      if (!workspace) return;
      const fetchedData: Methodology[] = await listMethodologies(
        workspace.slug,
      );
      setMethodologiesData(fetchedData);
    };
    fetchData();
  }, [workspace]);

  return (
    <Formik
      initialValues={formValues}
      validationSchema={validationSchema}
      onSubmit={onSubmit}
    >
      {({isValid, isSubmitting}) => {
        const disableSubmit = !isValid || isSubmitting;

        const options = methodologiesData.map(
          ({title: label, slug: value, description}) => ({
            label,
            value,
            description,
          }),
        );

        return (
          <Form>
            <Input label="Investigation Title" name="title" placeholder="" />

            <Textarea label="Description" name="description" placeholder="" />

            <DescriptionSelect
              label="Choose a methodology"
              name="methodology"
              options={options}
            />

            <FormActions
              submitLabel="Create Investigation"
              onCancel={onCancel}
              isDisabled={disableSubmit}
            />
          </Form>
        );
      }}
    </Formik>
  );
};

export default CreateInvestigationForm;
