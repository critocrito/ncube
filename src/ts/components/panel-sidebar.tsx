import c from "clsx";
import React, {useState} from "react";

import logoIcon from "../../../resources/public/images/logo_horizontal.svg";
import settingsIcon from "../../../resources/public/images/settings.svg";
import {useNcubeCtx, useWorkspaceCtx} from "../lib/context";
import PanelSidebarButton from "./panel-sidebar-button";
import PanelSidebarMenuItem from "./panel-sidebar-menu-item";
import WorkspaceSelect from "./workspace-select";

const PanelSidebar = () => {
  const [expanded, setExpanded] = useState(true);
  const [
    {
      context: {workspaces},
    },
    ncubeSend,
  ] = useNcubeCtx();
  const [workspaceState, workspaceSend] = useWorkspaceCtx();

  const toggleExpanded = () => setExpanded(!expanded);

  return (
    <aside className="relative h-screen">
      <div
        className={c("absolute inset-y-1/2", {
          "-right-4": expanded,
          "left-0": !expanded,
        })}
      >
        <PanelSidebarButton isExpanded={expanded} onClick={toggleExpanded} />
      </div>

      <div
        className={c(
          "flex-shrink-0 w-72 bg-white border-r border-solitude transition-all duration-300",
          {"-ml-72": !expanded},
        )}
      >
        <div className="h-screen flex flex-col">
          <div className="flex items-end flex-shrink-0 bg-sapphire h-24 px-2">
            <WorkspaceSelect
              workspaces={workspaces}
              selectedWorkspace={workspaceState.context.workspace}
              className="bg-sapphire text-white border border-sapphire hover:border-solitude"
              onChange={(workspace) => ncubeSend("SHOW_WORKSPACE", {workspace})}
            />
          </div>

          <nav className="mt-5 flex-1 px-4 space-y-4" aria-label="Sidebar">
            <PanelSidebarMenuItem
              kind="source"
              onClick={() => workspaceSend({type: "SOURCE"})}
            />
            <PanelSidebarMenuItem
              kind="data"
              onClick={() => workspaceSend({type: "DATA"})}
            />
            <PanelSidebarMenuItem
              kind="process"
              onClick={() => workspaceSend({type: "PROCESS"})}
            />
            <PanelSidebarMenuItem
              kind="investigation"
              onClick={() => workspaceSend({type: "INVESTIGATION"})}
            />
          </nav>

          <div className="flex-shrink-0 flex items-center justify-between px-4 py-2 border-t border-solitude bg-canvas">
            <button
              className="bg-canvas dim"
              onClick={() => ncubeSend("SHOW_DASHBOARD")}
            >
              <img className="h-10" src={logoIcon} alt="Ncube logo." />
            </button>
            <img className="w-8 h-8" src={settingsIcon} alt="Go to settings." />
          </div>
        </div>
      </div>
    </aside>
  );
};

export default PanelSidebar;
