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
    <div className="flex">
      <PanelSidebar />

      <div className="flex flex-column w-full">
        <PanelProcessConsole />

        <div className="px-16 py-8 space-y-8">
          <PanelBreadcrumbs />

          <PanelHeader header={header} description={description} />

          <div>{children}</div>
        </div>
      </div>
    </div>
  );
};

export default Panel;
