import {useMachine} from "@xstate/react";
import c from "classnames";
import parseISO from "date-fns/parseISO";
import {Form, Formik} from "formik";
import React from "react";

import {listAnnotations, setAnnotation} from "../../lib/http";
import {useServiceLogger} from "../../lib/utils";
import machine from "../../machines/annotation";
import {
  Annotation as AnnotationType,
  AnnotationReq,
  AnnotationSchema,
} from "../../types";
import BooleanSelect from "../boolean-select";
import Button from "../button";
import DatePicker from "../date-picker";
import Error from "../error";
import Input from "../input";
import Select from "../select";
import TextArea from "../text-area";
import Unreachable from "../unreachable";

interface AnnotationListProps {
  workspace: string;
  investigation: string;
  verification: number;
  schemas: AnnotationSchema[];
}

interface AnnotationItemProps {
  schema: AnnotationSchema;
  annotation?: AnnotationType;
  onSubmit: (annotation: AnnotationType) => Promise<void>;
}

export const AnnotationItem = ({
  schema,
  onSubmit,
  annotation,
}: AnnotationItemProps) => {
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

const AnnotationList = ({
  schemas,
  workspace,
  investigation,
  verification,
}: AnnotationListProps) => {
  const [state, send, service] = useMachine(machine, {
    services: {
      fetchAnnotations: (_ctx, _ev) =>
        listAnnotations(workspace, investigation, verification),
    },
  });

  useServiceLogger(service, machine.id);

  const {error, annotations} = state.context;

  if (state.matches("listAnnotations") || state.matches("home")) {
    return (
      <div className="flex flex-column">
        {schemas.map((schema) => {
          const annotation = annotations.find(({key}) => schema.key === key);

          return (
            <AnnotationItem
              key={schema.key}
              schema={schema}
              annotation={annotation}
              onSubmit={async (a: AnnotationReq) => {
                await setAnnotation(workspace, investigation, verification, a);
              }}
            />
          );
        })}
      </div>
    );
  }
  if (state.matches("error")) {
    return (
      <Error
        msg={error || "Failed to fetch annotations."}
        recover={() => send("SHOW_HOME")}
      />
    );
  }

  return (
    <Unreachable
      machine={machine.id}
      state={state.value}
      reset={() => send("SHOW_HOME")}
    />
  );
};

export default AnnotationList;
