import c from "classnames";
import parseISO from "date-fns/parseISO";
import {Form, Formik} from "formik";
import React from "react";

import BooleanSelect from "../common/boolean-select";
import Button from "../common/button";
import DatePicker from "../common/date-picker";
import Input from "../common/input";
import Select from "../common/select";
import TextArea from "../common/text-area";
import {
  Annotation as AnnotationType,
  AnnotationReq,
  AnnotationSchema,
} from "../types";

interface AnnotationProps {
  schema: AnnotationSchema;
  annotation?: AnnotationType;
  onSubmit: (annotation: AnnotationType) => Promise<void>;
}

const Annotation = ({schema, onSubmit, annotation}: AnnotationProps) => {
  const formValues = annotation
    ? {
        [schema.key]:
          schema.kind === "datetime"
            ? parseISO(annotation.value as string)
            : annotation.value,
        note: annotation.note ? annotation.note : "",
      }
    : {[schema.key]: ""};

  // The form values require some transformation to fit the correct request format.
  // {"some-annotation": "23"} -> {key: "some-annotation", value: "23"}
  //
  // IMPORTANT: If I add more fields to the annotation I probably need to update
  //            this function.
  const handleSubmit = ({
    note,
    ...rest
  }: {
    note?: string;
    [key: string]: unknown;
  }) => {
    const data = Object.keys(rest).reduce(
      (memo, key) => {
        return Object.assign(memo, {key, value: rest[key]});
      },
      {note, name: schema.name} as AnnotationReq,
    );
    return onSubmit(data);
  };

  return (
    <div className="ma3 w-100">
      <Formik
        initialValues={formValues}
        onSubmit={handleSubmit}
        enableReinitialize
      >
        {(formik) => {
          const isDisabled = !formik.isValid || formik.isSubmitting;

          let widget: React.ReactElement;

          switch (schema.kind) {
            case "boolean": {
              widget = <BooleanSelect label={schema.name} name={schema.key} />;
              break;
            }

            case "datetime": {
              widget = <DatePicker label={schema.name} name={schema.key} />;
              break;
            }

            case "selection": {
              widget = (
                <Select
                  label={schema.name}
                  name={schema.key}
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
              widget = <TextArea label={schema.name} name={schema.key} />;
              break;
            }

            default:
              widget = <Input label={schema.name} name={schema.key} />;
          }

          return (
            <Form>
              <div
                className={c(
                  "ba b--solitude flex flex-column w-100 pa2",
                  annotation ? "bg-white" : "bg-canvas",
                )}
              >
                {widget}

                <TextArea label="Notes" name="note" />
              </div>

              <div className="flex justify-between mt2">
                <Button
                  type="submit"
                  className="ml-auto"
                  kind="secondary"
                  disabled={isDisabled}
                >
                  {annotation ? "Update" : "Save"}
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
