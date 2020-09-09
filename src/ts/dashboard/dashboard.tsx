import React from "react";

import {Workspace} from "../types";
import WorkspaceActions from "./workspace-actions";
import WorkspaceHeader from "./workspace-header";
import WorkspaceList from "./workspace-list";

interface DashboardProps {
  workspaces: Workspace[];
  onShow: (workspace: Workspace) => void;
  onDelete: (workspace: Workspace) => void;
  onCreate: () => void;
  onLink: () => void;
}

const Dashboard = ({
  workspaces,
  onShow,
  onDelete,
  onCreate,
  onLink,
}: DashboardProps) => {
  const workspaceAction = (action: "show" | "delete", workspace: Workspace) => {
    if (action === "show") {
      onShow(workspace);
    } else if (action === "delete") {
      onDelete(workspace);
    }
  };

  return (
    <>
      <WorkspaceHeader />

      <WorkspaceList
        workspaces={workspaces}
        workspaceAction={workspaceAction}
      />

      <div className="ml-auto w-40 mt3">
        <WorkspaceActions onLink={onLink} onCreate={onCreate} />
      </div>
    </>
  );
};

export default Dashboard;
