import {Form, Formik} from "formik";
import React from "react";

import BooleanSelect from "../common/boolean-select";
import Button from "../common/button";
import DatePicker from "../common/date-picker";
import Input from "../common/input";
import Select from "../common/select";
import TextArea from "../common/text-area";
import {AnnotationSchema} from "../types";

interface AnnotationProps {
  schema: AnnotationSchema;
  onSubmit: (vals: Record<string, unknown>) => void;
}

const Annotation = ({schema, onSubmit}: AnnotationProps) => {
  const formValues = {[schema.key]: ""};

  let widget: React.ReactElement;

  switch (schema.kind) {
    case "boolean": {
      widget = (
        <BooleanSelect label={schema.name} name={schema.key || schema.name} />
      );
      break;
    }

    case "datetime": {
      widget = (
        <DatePicker label={schema.name} name={schema.key || schema.name} />
      );
      break;
    }

    case "selection": {
      widget = (
        <Select
          label={schema.name}
          name={schema.key || schema.name}
          options={
            (schema.selections || []).map((opt) => ({
              value: opt,
              label: opt,
            })) || []
          }
        />
      );
      break;
    }

    case "text": {
      widget = (
        <TextArea label={schema.name} name={schema.key || schema.name} />
      );
      break;
    }

    default:
      widget = <Input label={schema.name} name={schema.key || schema.name} />;
  }

  return (
    <div className="ma3 w-100">
      <Formik initialValues={formValues} onSubmit={onSubmit}>
        {(formik) => {
          const isDisabled = !formik.isValid || formik.isSubmitting;

          return (
            <Form>
              <div className="ba b--fair-pink flex flex-column w-100 pa2">
                {widget}

                <TextArea label="Notes" name="notes" />
              </div>

              <div className="flex justify-between mt2">
                <Button
                  type="submit"
                  className="ml-auto"
                  kind="secondary"
                  disabled={isDisabled}
                >
                  Save
                </Button>
              </div>
            </Form>
          );
        }}
      </Formik>
    </div>
  );
};

export default Annotation;
