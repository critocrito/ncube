import c from "classnames";
import React from "react";

import Button from "../common/button";

interface WorkspaceActionsProps {
  onLink: () => void;
  onCreate: () => void;
  className?: string;
}

const WorkspaceActions = ({
  onLink,
  onCreate,
  className,
}: WorkspaceActionsProps) => {
  return (
    <div className={c("flex justify-between", className)}>
      <Button kind="secondary" size="large" onClick={onLink}>
        Link Workspace
      </Button>
      <Button size="large" onClick={onCreate}>
        Create Workspace
      </Button>
    </div>
  );
};

export default WorkspaceActions;
