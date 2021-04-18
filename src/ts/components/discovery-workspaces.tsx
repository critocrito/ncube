import React, {useState} from "react";

import {Workspace} from "../types";
import Button from "./button";
import WorkspaceSelect from "./workspace-select";
import FormLabel from "./form-label";

interface WorkspacesProps {
  onNext: (workspace: Workspace) => void;
  workspaces: Workspace[];
}

const Workspaces = ({workspaces, onNext}: WorkspacesProps) => {
  const [selectedWorkspace, setSelectedWorkspace] = useState<Workspace>(
    workspaces[0],
  );

  return (
    <div className="flex flex-col">
      <FormLabel name="workspace" label="Select workspace" />

      <WorkspaceSelect
        className="border border-solitude text-black"
        selectedWorkspace={selectedWorkspace}
        workspaces={workspaces}
        onChange={(workspace) => setSelectedWorkspace(workspace)}
      />

      <Button
        className="mt-3 ml-auto"
        size="large"
        disabled={!selectedWorkspace}
        onClick={() => onNext(selectedWorkspace)}
      >
        Select
      </Button>
    </div>
  );
};

export default Workspaces;
