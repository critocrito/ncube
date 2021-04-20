import c from "clsx";
import parseISO from "date-fns/parseISO";
import {Form, Formik} from "formik";
import React from "react";

import BooleanSelect from "../components/boolean-select";
import Button from "../components/button";
import DatePicker from "../components/date-picker";
import Input from "../components/input";
import OptionsSelect from "../components/options-select";
import TextArea from "../components/text-area";
import {Annotation, AnnotationSchema} from "../types";

interface EditAnnotationProps {
  schema: AnnotationSchema;
  annotation?: Annotation;
  onSubmit: (annotation: Annotation) => Promise<void>;
}

const EditAnnotation = ({
  schema,
  onSubmit,
  annotation,
}: EditAnnotationProps) => {
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
      {note, name: schema.name} as Annotation,
    );
    return onSubmit(data);
  };

  return (
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
              <OptionsSelect
                label={schema.name}
                name={schema.key}
                options={(schema.selections || []).map((opt) => ({
                  value: opt,
                  label: opt,
                }))}
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
                "border border-solitude flex flex-col p-1.5",
                annotation ? "bg-white" : "bg-canvas",
              )}
            >
              {widget}

              <TextArea label="Notes" name="note" className="mt-3" />
            </div>

            <Button
              type="submit"
              className="mt-3 ml-auto"
              kind="secondary"
              disabled={isDisabled}
            >
              {annotation ? "Update" : "Save"}
            </Button>
          </Form>
        );
      }}
    </Formik>
  );
};

export default EditAnnotation;
