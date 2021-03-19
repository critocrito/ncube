import React from "react";

import DashboardActions from "../components/dashboard/actions";
import DashboardHeader from "../components/dashboard/header";
import DashboardWorkspaces from "../components/dashboard/workspaces";
import BasicPanel from "../components/panel-basic";
import {Workspace} from "../types";

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
    <BasicPanel>
      <div className="fl w-100 pa3">
        <DashboardHeader />

        <DashboardWorkspaces
          workspaces={workspaces}
          workspaceAction={workspaceAction}
        />

        <div className="ml-auto w-40 mt3">
          <DashboardActions onLink={onLink} onCreate={onCreate} />
        </div>
      </div>
    </BasicPanel>
  );
};

export default Dashboard;
