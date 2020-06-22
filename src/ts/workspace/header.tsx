import React from "react";

import {Workspace} from "../types";
import WorkspaceTag from "../common/workspace-tag";

interface HeaderProps {
  workspace: Workspace;
}

const Header = ({workspace}: HeaderProps) => {
  return (
    <div className="bb b--sapphire w-100 flex justify-between items-center">
      <div className="b text-medium sapphire ttu">{`Workspace: ${workspace.name}`}</div>
      <WorkspaceTag className="mb1" kind={workspace.kind} />
    </div>
  );
};

export default Header;
