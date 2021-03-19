/* eslint react/no-array-index-key: off */
import {Form, Formik} from "formik";
import React, {useEffect, useState} from "react";
import * as Yup from "yup";

import Button from "../components/button";
import {listInvestigations} from "../lib/http";
import {FormProps, Investigation} from "../types";
import InvestigationSelect from "./investigation-select";

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
            <InvestigationSelect
              name="investigation"
              investigations={investigationsData}
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
                Send To Verify
              </Button>
            </div>
          </Form>
        );
      }}
    </Formik>
  );
};

export default SendToVerificationForm;
