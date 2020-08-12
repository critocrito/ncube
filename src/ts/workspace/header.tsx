import React from "react";

import WorkspaceTag from "../common/workspace-tag";
import {useWorkspaceCtx} from "../context";
import {Workspace} from "../types";

interface HeaderProps {
  workspace: Workspace;
}

const Header = ({workspace}: HeaderProps) => {
  const [, workspaceSend] = useWorkspaceCtx();

  return (
    <div className="bb b--sapphire w-100 flex justify-between items-center pb2">
      <div className="b text-medium sapphire ttu w-30">
        <button
          className="b--none bg-transparent pointer b sapphire ttu ma0 pa0"
          onClick={() => workspaceSend("OVERVIEW")}
        >
          {`Workspace: ${workspace.name}`}
        </button>
      </div>
      <WorkspaceTag className="mb1" kind={workspace.kind} />
    </div>
  );
};

export default Header;
