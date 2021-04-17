import React from "react";

import Button from "./button";

interface SourcesActionsProps {
  onCreate: () => void;
}

const SourcesActions = ({onCreate}: SourcesActionsProps) => {
  return (
    <Button className="flex items-center" onClick={onCreate} kind="secondary">
      <span>Add New</span>
    </Button>
  );
};

export default SourcesActions;
