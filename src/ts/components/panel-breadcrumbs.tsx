import React from "react";

import {useWorkspaceCtx} from "../lib/context";
import WorkspaceTag from "./workspace-tag";

const PanelBreadcrumbs = () => {
  const [
    {
      context: {workspace},
    },
    workspaceSend,
  ] = useWorkspaceCtx();

  return (
    <div className="border-b border-sapphire flex sm:justify-between items-center py-1.5">
      <button
        className="font-bold text-sapphire text-sm uppercase dim"
        onClick={() => workspaceSend({type: "OVERVIEW"})}
      >
        {`Workspace: ${workspace.name}`}
      </button>

      <WorkspaceTag kind={workspace.kind} />
    </div>
  );
};

export default PanelBreadcrumbs;
