import React from "react";

import {Workspace} from "../types";

interface WorkspaceSelectorProps {
  workspaces: Workspace[];
  onChange: (w: Workspace) => void;
  selectedWorkspace?: Workspace;
}

const WorkspaceSelector = ({
  workspaces,
  selectedWorkspace,
  onChange,
}: WorkspaceSelectorProps) => {
  return (
    <select
      defaultValue={selectedWorkspace?.slug}
      className="w-100 mt5 b white b--sapphire bg-sapphire"
      onChange={(ev) => {
        const workspace = workspaces.find(({slug}) => slug === ev.target.value);
        if (workspace) onChange(workspace);
      }}
    >
      {workspaces.map(({name, slug}) => (
        <option key={slug} value={slug} label={name} />
      ))}
    </select>
  );
};

export default WorkspaceSelector;
