import {useMachine} from "@xstate/react";
import React from "react";

import Error from "../common/error";
import Fatal from "../common/fatal";
import {listAnnotations, setAnnotation} from "../http";
import machine from "../machines/annotation";
import {AnnotationReq, AnnotationSchema} from "../types";
import {useServiceLogger} from "../utils";
import Annotation from "./annotation";

interface AnnotationListProps {
  workspace: string;
  investigation: string;
  verification: number;
  schemas: AnnotationSchema[];
}

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

  switch (true) {
    case state.matches("listAnnotations"):
    case state.matches("home"): {
      return (
        <div className="flex flex-column">
          {schemas.map((schema) => {
            const annotation = annotations.find(({key}) => schema.key === key);

            return (
              <Annotation
                key={schema.key}
                schema={schema}
                annotation={annotation}
                onSubmit={async (a: AnnotationReq) => {
                  await setAnnotation(
                    workspace,
                    investigation,
                    verification,
                    a,
                  );
                }}
              />
            );
          })}
        </div>
      );
    }

    case state.matches("error"):
      return (
        <Error
          msg={error || "Failed to fetch annotations."}
          recover={() => send("SHOW_HOME")}
        />
      );

    default:
      return (
        <Fatal
          msg={`Annotation list route didn't match any valid state: ${state.value}`}
          reset={() => send("SHOW_HOME")}
        />
      );
  }
};

export default AnnotationList;
