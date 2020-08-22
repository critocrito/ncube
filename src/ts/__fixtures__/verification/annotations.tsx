import React from "react";

import Annotation from "../../investigation/annotation";
import {AnnotationSchema} from "../../types";
import data from "../annotations.json";

const annotations: React.ReactElement[] = [];

// FIXME: wrap and set state to print on screen
const submitHandler = (values: Record<string, unknown>) => console.log(values);

data.forEach((annotation) => {
  annotations.push(
    <div key={annotation.key} className="flex items-center w-100">
      <Annotation
        schema={(annotation as unknown) as AnnotationSchema}
        onSubmit={submitHandler}
      />
    </div>,
  );
});

export default (
  <div className="noto lh-copy pa2 flex flex-column">{annotations}</div>
);
