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
      <WorkspaceSelector
        selectedWorkspace={selectedWorkspace}
        workspaces={workspaces}
        onChange={(workspace: Workspace) => {
          setSelectedWorkspace(workspace);
        }}
      />
      <div className="flex justify-around mt2">
        <Button
          size="large"
          onClick={() => selectedWorkspace && onNext(selectedWorkspace)}
        >
          Select
        </Button>
      </div>
    </div>
  );
};

export default Workspaces;
