import {Form, Formik} from "formik";
import React, {useEffect, useState} from "react";
import * as Yup from "yup";

import Button from "../components/button";
import Input from "../components/input";
import Textarea from "../components/text-area";
import {listMethodologies} from "../lib/http";
import {FormProps, Methodology} from "../types";
import MethodologySelect from "./methodology-select";

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

        return (
          <Form>
            <Input label="Investigation Title" name="title" placeholder="" />

            <Textarea label="Description" name="description" placeholder="" />

            <MethodologySelect methodologies={methodologiesData} />

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
                Create Investigation
              </Button>
            </div>
          </Form>
        );
      }}
    </Formik>
  );
};

export default CreateInvestigationForm;
