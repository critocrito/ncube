import React from "react";

import Button from "./button";

interface DashboardActionsProps {
  onLink: () => void;
  onCreate: () => void;
}

const DashboardActions = ({onLink, onCreate}: DashboardActionsProps) => {
  return (
    <>
      <Button kind="secondary" size="large" onClick={onLink}>
        Link Workspace
      </Button>
      <Button size="large" onClick={onCreate}>
        Create Workspace
      </Button>
    </>
  );
};

export default DashboardActions;
