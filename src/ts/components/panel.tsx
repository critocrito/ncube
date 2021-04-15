import React from "react";

import PanelProcessConsole from "./panel-process-console";
import PanelBreadcrumbs from "./panel-breadcrumbs";
import PanelSidebar from "./panel-sidebar";
import PanelHeader from "./panel-header";

interface PanelProps {
  header: string;
  description?: string;
  children: JSX.Element;
}

const Panel = ({children, header, description}: PanelProps) => {
  return (
    <div className="flex h-screen overflow-x-hidden">
      <PanelSidebar />

      <div className="flex-1">
        <PanelProcessConsole />

        <div className="px-16 py-8 space-y-8 h-full overflow-y-auto">
          <PanelBreadcrumbs />

          <PanelHeader header={header} description={description} />

          <div>{children}</div>
        </div>
      </div>
    </div>
  );
};

export default Panel;
