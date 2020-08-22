import React, {useEffect, useState} from "react";
import {EventObject} from "xstate";

import DataDetails from "../database/details";
import {showUnit} from "../http";
import {AnnotationSchema, SegmentUnit, Unit, Workspace} from "../types";
import Annotation from "./annotation";

interface VerificationDetailsProps<
  TContext extends Record<string, unknown>,
  TEvent extends EventObject
> {
  workspace: Workspace;
  unit: SegmentUnit<TContext, TEvent>;
}

const VerificationDetails = <
  TContext extends Record<string, unknown>,
  TEvent extends EventObject
>({
  workspace: {slug},
  unit: {state, id},
}: VerificationDetailsProps<TContext, TEvent>) => {
  const [unit, setUnit] = useState<Unit | undefined>();

  const annotations: AnnotationSchema[] = Object.keys(state.meta).reduce(
    (memo, key) => memo.concat(state.meta[key].annotations ?? []),
    [],
  );

  useEffect(() => {
    const f = async () => {
      const data = await showUnit(slug, id);
      setUnit(data);
    };
    f();
  }, [slug, id]);

  return (
    <div className="flex">
      {unit && (
        <div className="w-50">
          <DataDetails unit={unit} />
        </div>
      )}
      {unit && (
        <div className="w-50 flex flex-column">
          {annotations.map((a) => (
            <Annotation
              key={a.key}
              schema={a}
              onSubmit={(values) => console.log(values)}
            />
          ))}
        </div>
      )}
    </div>
  );
};

export default VerificationDetails;
