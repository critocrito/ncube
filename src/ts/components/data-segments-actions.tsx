import React from "react";

import Button from "./button";

interface DataSegmentsActionsProps {
  onVerify: () => void;
  onDelete: () => void;
}

const DataSegmentsActions = ({
  onVerify,
  onDelete,
}: DataSegmentsActionsProps) => {
  return (
    <div className="flex flex-col items-center space-y-4">
      <Button size="small" onClick={onVerify}>
        Verify
      </Button>

      <Button size="small" kind="caution" onClick={onDelete}>
        Remove
      </Button>
    </div>
  );
};

export default DataSegmentsActions;
