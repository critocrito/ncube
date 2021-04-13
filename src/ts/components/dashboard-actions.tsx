import c from "classnames";
import React from "react";

import Button from "./button";

interface DashboardActionsProps {
  onLink: () => void;
  onCreate: () => void;
  className?: string;
}

const DashboardActions = ({
  onLink,
  onCreate,
  className,
}: DashboardActionsProps) => {
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

export default DashboardActions;
