import React from "react";

import ExpandButton from "./expand-button";

interface ProcessesActionsProps {
  onRun: () => void;
}

const ProcessesActions = ({onRun}: ProcessesActionsProps) => {
  return (
    <ExpandButton label="Preserve">
      {(Item) => {
        return (
          <>
            <Item disabled onClick={onRun}>
              Selected Sources
            </Item>
            <Item disabled onClick={onRun}>
              New Sources
            </Item>
            <Item onClick={onRun}>All Sources</Item>
          </>
        );
      }}
    </ExpandButton>
  );
};

export default ProcessesActions;
