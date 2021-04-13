import React, {useState} from "react";

import {Workspace} from "../types";
import Button from "./button";
import WorkspaceSelector from "./workspace-selector";

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
      <p className="mb2">Select workspace</p>
      <WorkspaceSelector
        className="workspace-select pa2 mt3 mb5 bg-white ba b--solitude black"
        selectedWorkspace={selectedWorkspace}
        workspaces={workspaces}
        onChange={(workspace: Workspace) => {
          setSelectedWorkspace(workspace);
        }}
      />

      <Button
        className="mt5"
        size="large"
        onClick={() => selectedWorkspace && onNext(selectedWorkspace)}
      >
        Select
      </Button>
    </div>
  );
};

export default Workspaces;
