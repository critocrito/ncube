import React from "react";

import {useWorkspaceCtx} from "../lib/context";
import {Workspace} from "../types";
import WorkspaceTag from "./workspace-tag";

interface PanelHeaderProps {
  workspace: Workspace;
}

const PanelHeader = ({workspace}: PanelHeaderProps) => {
  const [, workspaceSend] = useWorkspaceCtx();

  return (
    <div className="bb b--sapphire w-100 flex justify-between items-center pb2">
      <div className="b text-medium sapphire ttu w-30">
        <button
          className="b--none bg-transparent pointer b sapphire ttu ma0 pa0 dim"
          onClick={() => workspaceSend({type: "OVERVIEW"})}
        >
          {`Workspace: ${workspace.name}`}
        </button>
      </div>
      <WorkspaceTag className="mb1" kind={workspace.kind} />
    </div>
  );
};

export default PanelHeader;
