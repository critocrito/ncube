/* eslint react/no-array-index-key: off */
import {Form, Formik} from "formik";
import React, {useEffect, useState} from "react";
import * as Yup from "yup";

import DescriptionSelect from "../components/description-select";
import FormActions from "../components/form-actions";
import {listInvestigations} from "../lib/http";
import {FormProps, Investigation} from "../types";

type SendToVerificationFormProps<
  SendToVerificationFormValues
> = FormProps<SendToVerificationFormValues>;

export interface SendToVerificationFormValues {
  investigation: string;
}

export const defaultValues: SendToVerificationFormValues = {
  investigation: "",
};

export const validationSchema = Yup.object({
  investigation: Yup.string().required("This field is required."),
});

const SendToVerificationForm = ({
  initialValues = defaultValues,
  onCancel,
  onSubmit,
  workspace,
}: SendToVerificationFormProps<SendToVerificationFormValues>) => {
  const [investigationsData, setInvestigationsData] = useState<Investigation[]>(
    [],
  );
  const formValues = {...defaultValues, ...initialValues};

  useEffect(() => {
    const fetchData = async () => {
      if (!workspace) return;
      const fetchedData = await listInvestigations(workspace.slug);
      setInvestigationsData(fetchedData);
    };
    fetchData();
  }, [workspace]);

  const options = investigationsData.map(
    ({title: label, slug: value, description}) => ({
      label,
      value,
      description,
    }),
  );

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
            <DescriptionSelect
              label="Choose an investigation"
              name="investigation"
              options={options}
            />

            <FormActions
              submitLabel="Send To Verify"
              onCancel={onCancel}
              isDisabled={disableSubmit}
            />
          </Form>
        );
      }}
    </Formik>
  );
};

export default SendToVerificationForm;
