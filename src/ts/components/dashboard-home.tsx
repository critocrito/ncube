import React from "react";

import {voidFn} from "../lib/utils";
import {Workspace} from "../types";
import ActionsLayout from "./actions-layout";
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
    <>
      <DashboardHeader />

      <DashboardTable
        workspaces={workspaces}
        onShow={onShow}
        onDelete={onDelete}
      />

      <ActionsLayout align="right">
        <DashboardActions onLink={onLink} onCreate={onCreate} />
      </ActionsLayout>
    </>
  );
};

export default DashboardHome;
