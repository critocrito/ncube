import React from "react";

import {Annotation, AnnotationSchema} from "../types";
import EditAnnotation from "../forms/edit-annotations";

interface AnnotationListProps {
  schemas: AnnotationSchema[];
  annotations: Annotation[];
  onUpdateAnnotation: (a: Annotation) => void;
}

const AnnotationList = ({
  schemas,
  annotations,
  onUpdateAnnotation,
}: AnnotationListProps) => {
  return (
    <div className="flex flex-col space-y-6">
      {schemas.map((schema) => {
        const annotation = annotations.find(({key}) => schema.key === key);

        return (
          <EditAnnotation
            key={schema.key}
            schema={schema}
            annotation={annotation}
            onSubmit={async (a) => {
              onUpdateAnnotation(a);
            }}
          />
        );
      })}
    </div>
  );
};

export default AnnotationList;
