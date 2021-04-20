import React from "react";

import PanelBreadcrumbs from "./panel-breadcrumbs";
import PanelHeader from "./panel-header";
import PanelProcessConsole from "./panel-process-console";
import PanelSidebar from "./panel-sidebar";

interface PanelProps {
  header: string;
  description?: string;
  children: JSX.Element;
}

const Panel = ({children, header, description}: PanelProps) => {
  return (
    <div className="flex">
      <PanelSidebar />

      <div className="flex-auto">
        <PanelProcessConsole />

        <div className="px-16 py-8 space-y-8 h-screen overflow-y-auto">
          <PanelBreadcrumbs />

          <PanelHeader header={header} description={description} />

          <main>{children}</main>
        </div>
      </div>
    </div>
  );
};

export default Panel;
