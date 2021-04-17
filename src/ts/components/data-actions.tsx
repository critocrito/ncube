import React from "react";

import Button from "./button";

interface DataActionsProps {
  hasSegment: boolean;
  isDisabled: boolean;
  onCreateSegment: () => void;
  onUpdateSegment: () => void;
}

const DataActions = ({
  hasSegment,
  isDisabled,
  onCreateSegment,
  onUpdateSegment,
}: DataActionsProps) => {
  return hasSegment ? (
    <Button onClick={onUpdateSegment} size="large" disabled={isDisabled}>
      Update Segment
    </Button>
  ) : (
    <Button onClick={onCreateSegment} size="large" disabled={isDisabled}>
      Save Segment
    </Button>
  );
};

export default DataActions;
