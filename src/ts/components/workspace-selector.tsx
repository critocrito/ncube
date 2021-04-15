import c from "clsx";
import React from "react";

import {Workspace} from "../types";

interface WorkspaceSelectorProps {
  workspaces: Workspace[];
  onChange: (w: Workspace) => void;
  selectedWorkspace?: Workspace;
  className?: string;
}

const WorkspaceSelector = ({
  workspaces,
  selectedWorkspace,
  onChange,
  className,
}: WorkspaceSelectorProps) => {
  return (
    <select
      defaultValue={selectedWorkspace?.slug}
      className={c(className, "font-bold px-0")}
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
