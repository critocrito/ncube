import React from "react";

import {
  Annotation,
  AnnotationReq,
  AnnotationSchema,
  SegmentUnit,
  Unit,
} from "../types";
import DataDetails from "./data-details";
import DefinitionItem from "./definition-item";
import VerificationAnnotations from "./verification-annotations";

interface VerificationDetailsProps {
  segmentUnit: SegmentUnit;
  unit: Unit;
  annotations: Annotation[];
  onUpdateAnnotation: (a: AnnotationReq) => void;
}

const VerificationDetails = ({
  segmentUnit: {state},
  unit,
  annotations,
  onUpdateAnnotation,
}: VerificationDetailsProps) => {
  const schemas = Object.keys(state.meta).reduce(
    (memo, key) => [...memo, ...(state.meta[key].annotations ?? [])],
    [] as AnnotationSchema[],
  );

  return (
    <div className="flex">
      <div className="w-50">
        <DataDetails unit={unit} />

        <div className="flex justify-between items-center mt3">
          <span className="ttu w-10 b text-md">VerificationAnnotations</span>
          <hr className="w-80" />
        </div>

        <ul className="pl0 list">
          {annotations.map((a) => {
            let {value} = a;

            if (typeof value === "boolean" && value) {
              value = "Yes";
            } else if (typeof value === "boolean" && !value) {
              value = "No";
            }

            return (
              <li key={a.key}>
                <DefinitionItem item={a.name} value={value as string} />
              </li>
            );
          })}
        </ul>
      </div>

      <div className="w-50">
        <h4 className="header4">Edit VerificationAnnotations</h4>

        <VerificationAnnotations
          schemas={schemas}
          annotations={annotations}
          onUpdateAnnotation={onUpdateAnnotation}
        />
      </div>
    </div>
  );
};

export default VerificationDetails;
