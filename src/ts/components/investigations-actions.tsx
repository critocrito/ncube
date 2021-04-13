import React from "react";

import Button from "./button";

interface InvestigationsActionsProps {
  onCreate: () => void;
}

const InvestigationsActions = ({onCreate}: InvestigationsActionsProps) => {
  return (
    <div className="flex mb3">
      <Button className="ml-auto" size="large" onClick={onCreate}>
        Create New
      </Button>
    </div>
  );
};

export default InvestigationsActions;
