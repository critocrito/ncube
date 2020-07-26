import React, {useState} from "react";

import Button from "../common/button";
import WorkspaceSelector from "../common/workspace-selector";
import {Workspace} from "../types";

interface WorkspacesProps {
  onNext: (workspace: Workspace) => void;
  workspaces: Workspace[];
}

const Workspaces = ({workspaces, onNext}: WorkspacesProps) => {
  const [selectedWorkspace, setSelectedWorkspace] = useState<
    Workspace | undefined
  >(workspaces[0]);

  return (
    <div className="flex flex-column">
      <p>Select workspace</p>
      <WorkspaceSelector
        className="pa2 mt3 mb5 bg-white ba--solitude"
        selectedWorkspace={selectedWorkspace}
        workspaces={workspaces}
        onChange={(workspace: Workspace) => {
          setSelectedWorkspace(workspace);
        }}
      />

      <Button
        className="mt4"
        size="large"
        onClick={() => selectedWorkspace && onNext(selectedWorkspace)}
      >
        Select
      </Button>
    </div>
  );
};

export default Workspaces;
