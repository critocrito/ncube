import React from "react";

import {voidFn} from "../lib/utils";
import {Workspace} from "../types";
import DashboardActions from "./dashboard-actions";
import DashboardHeader from "./dashboard-header";
import DashboardTable from "./dashboard-table";

interface DashboardHomeProps {
  workspaces: Workspace[];
  onShow?: (workspace: Workspace) => void;
  onDelete?: (workspace: Workspace) => void;
  onLink?: () => void;
  onCreate?: () => void;
}

const DashboardHome = ({
  workspaces,
  onShow = voidFn,
  onDelete = voidFn,
  onLink = voidFn,
  onCreate = voidFn,
}: DashboardHomeProps) => {
  return (
    <div className="fl w-100 pa3">
      <DashboardHeader />

      <DashboardTable
        workspaces={workspaces}
        onShow={onShow}
        onDelete={onDelete}
      />

      <div className="ml-auto w-40 mt3">
        <DashboardActions onLink={onLink} onCreate={onCreate} />
      </div>
    </div>
  );
};

export default DashboardHome;
