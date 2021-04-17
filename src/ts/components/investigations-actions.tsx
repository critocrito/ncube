import React from "react";

import Button from "./button";

interface InvestigationsActionsProps {
  onCreate: () => void;
}

const InvestigationsActions = ({onCreate}: InvestigationsActionsProps) => {
  return (
    <Button size="large" onClick={onCreate}>
      Create New
    </Button>
  );
};

export default InvestigationsActions;
