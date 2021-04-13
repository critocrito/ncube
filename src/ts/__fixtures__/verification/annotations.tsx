import React from "react";

import {AnnotationItem} from "../../components/verification-annotations";
import {Annotation as AnnotationType} from "../../types";
import {annotations} from "../data";

const elements: React.ReactElement[] = [];

// FIXME: wrap and set state to print on screen
const submitHandler = (values: AnnotationType) => {
  console.log(values);
  return Promise.resolve();
};

annotations.forEach((annotation) => {
  elements.push(
    <div key={annotation.key} className="flex items-center w-100">
      <AnnotationItem schema={annotation} onSubmit={submitHandler} />
    </div>,
  );
});

export default (
  <div className="noto lh-copy pa2 flex flex-column">{elements}</div>
);
