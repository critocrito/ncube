import {Form, Formik} from "formik";
import React from "react";
import * as Yup from "yup";

import Button from "../components/button";
import Input from "../components/input";

interface ConfigureProcessFormProps {
  template: Record<string, string>;
  values: Record<string, string>;
  onSubmit: (values: Record<string, string>) => void;
  onCancel: () => void;
}

const ConfigureProcessForm = ({
  template,
  values,
  onSubmit,
  onCancel,
}: ConfigureProcessFormProps) => {
  const initialValues = Object.keys(template).reduce((memo, key) => {
    const value = values[key] ? values[key] : "";
    return Object.assign(memo, {[key]: value});
  }, {});

  const validationSchema = Yup.object(
    Object.keys(template).reduce((memo, key) => {
      return Object.assign(memo, {[key]: Yup.string().required()});
    }, {}),
  );

  return (
    <Formik
      initialValues={initialValues}
      validationSchema={validationSchema}
      onSubmit={onSubmit}
    >
      {({isSubmitting, isValid}) => {
        const disableSubmit = !isValid || isSubmitting;

        return (
          <Form>
            {Object.keys(template).map((key) => {
              const label = template[key];
              return <Input key={key} label={label} name={key} />;
            })}

            <div className="flex">
              <Button
                className="ml-auto"
                size="normal"
                kind="secondary"
                onClick={onCancel}
              >
                Cancel
              </Button>
              <Button
                className="ml2"
                size="normal"
                type="submit"
                disabled={disableSubmit}
              >
                Set up
              </Button>
            </div>
          </Form>
        );
      }}
    </Formik>
  );
};

export default ConfigureProcessForm;
