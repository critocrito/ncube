import React from "react";

import CreateSegmentForm, {
  CreateSegmentFormValues,
} from "../forms/create-segment";
import {createSegment} from "../lib/http";
import {Workspace} from "../types";
import FormHandler from "./form-handler";

interface DataSegmentsCreateProps {
  workspace: Workspace;
  initialValues: Partial<CreateSegmentFormValues>;
  onDone: () => void;
}

const DataSegmentsCreate = ({
  workspace: {slug: workspace},
  initialValues,
  onDone,
}: DataSegmentsCreateProps) => {
  return (
    <div className="flex flex-col">
      <p>Add a new segment for your workspace.</p>

      <FormHandler
        onSave={(values) => createSegment(workspace, values)}
        onDone={onDone}
        Form={CreateSegmentForm}
        initialValues={initialValues}
      />
    </div>
  );
};

export default DataSegmentsCreate;
