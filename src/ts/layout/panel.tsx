import React from "react";

import Navbar from "../common/navbar";
import Sidebar from "../common/sidebar";
import {Workspace} from "../types";
import Header from "../workspace/header";

interface PanelProps {
  header: string;
  description?: string;
  workspaces: Workspace[];
  workspace: Workspace;
  children: JSX.Element;
}

const Panel = ({
  children,
  header,
  description,
  workspaces,
  workspace,
}: PanelProps) => {
  return (
    <div className="flex">
      <Sidebar workspaces={workspaces} />
      <div className="w-100">
        <Navbar />
        <div className="ph4 pv3 mw8 center">
          <Header workspace={workspace} />
          <div>
            <h1 className="header1">{header}</h1>
            <p className="text-medium">{description}</p>
          </div>
          <div className="cf w-100 pv3">{children}</div>
        </div>
      </div>
    </div>
  );
};

export default Panel;
