import React from "react";

import {Workspace} from "../../types";
import Navbar from "../base/navbar";
import Sidebar from "../base/sidebar";
import Header from "../workspace/header";

interface PanelProps {
  workspaces: Workspace[];
  workspace: Workspace;
  children: JSX.Element;
}

const Panel = ({children, workspaces, workspace}: PanelProps) => {
  return (
    <div className="flex">
      <Sidebar workspaces={workspaces} />
      <div className="w-100">
        <Navbar />
        <div className="pa3 ma2 mw8 center">
          <Header workspace={workspace} />
          <div>
            <h1 className="header1">{workspace.name}</h1>
            <p className="text-medium">{workspace.description}</p>
          </div>
          <div className="cf w-100 pa2">{children}</div>
        </div>
      </div>
    </div>
  );
};

export default Panel;
