import React from "react";

import UpdateSegmentForm, {
  UpdateSegmentFormValues,
} from "../forms/update-segment";
import {updateSegment} from "../lib/http";
import {Segment, Workspace} from "../types";
import FormHandler from "./form-handler";

interface DataSegmentsUpdateProps {
  workspace: Workspace;
  segment: Segment;
  initialValues: UpdateSegmentFormValues;
  onDone: () => void;
}

const DataSegmentsUpdate = ({
  workspace: {slug: workspace},
  segment: {slug: segment},
  initialValues,
  onDone,
}: DataSegmentsUpdateProps) => {
  return (
    <div className="flex flex-col">
      <h4 className="header4 mb-3">Modify this segment.</h4>

      <FormHandler
        onSave={(values) => updateSegment(workspace, segment, values)}
        onDone={onDone}
        Form={UpdateSegmentForm}
        initialValues={initialValues}
      />
    </div>
  );
};

export default DataSegmentsUpdate;
