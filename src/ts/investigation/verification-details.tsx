import React, {useEffect, useState} from "react";
import {EventObject} from "xstate";

import DefinitionItem from "../database/definition-item";
import DataDetails from "../database/details";
import {listAnnotations, showUnit} from "../http";
import {
  Annotation as AnnotationType,
  AnnotationSchema,
  Investigation,
  SegmentUnit,
  Unit,
  Workspace,
} from "../types";
import AnnotationList from "./annotation-list";

interface VerificationDetailsProps<
  TContext extends Record<string, unknown>,
  TEvent extends EventObject
> {
  workspace: Workspace;
  investigation: Investigation;
  unit: SegmentUnit<TContext, TEvent>;
}

const VerificationDetails = <
  TContext extends Record<string, unknown>,
  TEvent extends EventObject
>({
  workspace: {slug},
  investigation: {slug: investigationSlug},
  unit: {state, id, verification},
}: VerificationDetailsProps<TContext, TEvent>) => {
  const [unit, setUnit] = useState<Unit | undefined>();
  const [annotations, setAnnotations] = useState<AnnotationType[]>([]);

  const schemas = Object.keys(state.meta).reduce(
    (memo, key) => [...memo, ...(state.meta[key].annotations ?? [])],
    [] as AnnotationSchema[],
  );

  useEffect(() => {
    const f = async () => {
      const data = await showUnit(slug, id);
      setUnit(data);
    };
    f();
  }, [slug, id]);

  useEffect(() => {
    const f = async () => {
      const data = await listAnnotations(slug, investigationSlug, id);
      setAnnotations(data);
    };
    f();
  }, [slug, investigationSlug, id]);

  return (
    <div className="flex">
      {unit && (
        <>
          <div className="w-50">
            <DataDetails unit={unit} />

            <div className="flex justify-between items-center mt3">
              <span className="ttu w-10 b text-medium">Annotations</span>
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
            <h4 className="header4">Edit Annotations</h4>

            <AnnotationList
              schemas={schemas}
              workspace={slug}
              investigation={investigationSlug}
              verification={verification}
            />
          </div>
        </>
      )}
    </div>
  );
};

export default VerificationDetails;
