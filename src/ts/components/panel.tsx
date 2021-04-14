import React from "react";

import {useNcubeCtx, useWorkspaceCtx} from "../lib/context";
import Navbar from "./navbar";
import PanelHeader from "./panel-header";
import Sidebar from "./sidebar";

interface PanelProps {
  header: string;
  description?: string;
  children: JSX.Element;
}

const Panel = ({children, header, description}: PanelProps) => {
  const [
    {
      context: {workspaces},
    },
  ] = useNcubeCtx();

  const [
    {
      context: {workspace},
    },
  ] = useWorkspaceCtx();

  return (
    <div className="flex">
      <Sidebar workspaces={workspaces} />
      <div className="w-100 flex flex-column">
        <Navbar />
        <div className="ml4 mr4">
          <div className="ph4 pv3 center">
            <PanelHeader workspace={workspace} />

            <div>
              <h1 className="header1">{header}</h1>
              <p className="text-md">{description}</p>
            </div>

            <div className="cf w-100 pv3">{children}</div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default Panel;
