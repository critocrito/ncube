import c from "clsx";
import React from "react";

import Button from "./button";

interface SourcesActionsProps {
  onCreate: () => void;
  className?: string;
}

const SourcesActions = ({onCreate, className}: SourcesActionsProps) => {
  const classes = c("flex items-center mb3 justify-between", className);

  return (
    <div className={classes}>
      <Button className="flex items-center" onClick={onCreate} kind="secondary">
        <span>Add New</span>
      </Button>
    </div>
  );
};

export default SourcesActions;
