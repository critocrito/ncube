import React from "react";

import {
  Annotation,
  AnnotationReq,
  AnnotationSchema,
  SegmentUnit,
  Unit,
} from "../types";
import DataDetails from "./data-details";
import Description from "./description";
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

  const items = annotations.map(({name: label, value, key}) => {
    if (typeof value === "boolean") {
      return {label, key, value: value ? "Yes" : "No"};
    }
    return {label, key, value: value as string};
  });

  return (
    <div className="flex space-x-3">
      <div className="w-1/2">
        <DataDetails unit={unit} />

        <h5 className="header5 font-bold text-sapphire uppercase mt-4">
          Annotations
        </h5>

        <Description items={items} />
      </div>

      <div className="w-1/2">
        <h4 className="header4">Edit Annotations</h4>

        <h5 className="header5 font-bold text-sapphire uppercase mt-4">
          &nbsp;
        </h5>

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
