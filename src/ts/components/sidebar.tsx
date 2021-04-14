import c from "clsx";
import React, {useState} from "react";

import chevronLeftIcon from "../../../resources/public/images/icon_chevron_left.svg";
import chevronRightIcon from "../../../resources/public/images/icon_chevron_right.svg";
import dataIcon from "../../../resources/public/images/icon_data.svg";
import investigationIcon from "../../../resources/public/images/icon_investigation.svg";
import processIcon from "../../../resources/public/images/icon_process.svg";
import sourceIcon from "../../../resources/public/images/icon_source.svg";
import logoIcon from "../../../resources/public/images/logo_horizontal.svg";
import settingsIcon from "../../../resources/public/images/settings.svg";
import {useNcubeCtx, useWorkspaceCtx} from "../lib/context";
import {Workspace} from "../types";
import WorkspaceSelector from "./workspace-selector";

interface SidebarProps {
  workspaces: Workspace[];
}

const Sidebar = ({workspaces}: SidebarProps) => {
  const [expanded, setExpanded] = useState(true);
  const [, ncubeSend] = useNcubeCtx();
  const [workspaceState, workspaceSend] = useWorkspaceCtx();

  const toggleExpanded = () => setExpanded(!expanded);

  const buttonLabel = expanded ? (
    <img src={chevronLeftIcon} alt="close sidebar" className="h1 w1" />
  ) : (
    <img src={chevronRightIcon} alt="open sidebar" className="h1 w1" />
  );

  return (
    <>
      <div
        className="fixed cursor"
        style={{
          top: "50%",
          left: expanded ? "238px" : "0px",
        }}
      >
        <button
          className="text-sapphire b h2 w2 br-100 shadow-1 b--none bg-canvas flex flex-column justify-around items-center pointer dim"
          onClick={toggleExpanded}
        >
          {buttonLabel}
        </button>
      </div>
      <div className="bg-white b--solitude">
        <div className={c(!expanded ? "dn" : undefined)}>
          <div className="w5 vh-100 flex flex-column ">
            <div className="fixed top-0 w5">
              <div className="pa2 bg-sapphire h4">
                <div className="mt5">
                  <WorkspaceSelector
                    workspaces={workspaces}
                    selectedWorkspace={workspaceState.context.workspace}
                    className="workspace-select w-100 b bg-sapphire ba b--sapphire white pointer"
                    onChange={(workspace) =>
                      ncubeSend("SHOW_WORKSPACE", {workspace})
                    }
                  />
                </div>
              </div>

              <div className="flex flex-column justify-between h-100">
                <div className="pa2">
                  <ul className="list pl0 ">
                    <li>
                      <button
                        className="flex items-center b--none bg-white pointer dim"
                        onClick={() => workspaceSend({type: "SOURCE"})}
                      >
                        <img
                          height="20px"
                          width="20px"
                          src={sourceIcon}
                          alt="Go to workspace sources."
                        />
                        <span className="ml2">Sources</span>
                      </button>
                    </li>

                    <li className="mt3">
                      <button
                        className="flex items-center b--none bg-white pointer dim"
                        onClick={() => workspaceSend({type: "DATA"})}
                      >
                        <img
                          height="20px"
                          width="20px"
                          src={dataIcon}
                          alt="Go to workspace data."
                        />
                        <span className="ml2">Data</span>
                      </button>
                    </li>

                    <li className="mt3">
                      <button
                        className="flex items-center b--none bg-white pointer dim"
                        onClick={() => workspaceSend({type: "PROCESS"})}
                      >
                        <img
                          height="20px"
                          width="20px"
                          src={processIcon}
                          alt="Go to workspace processes."
                        />
                        <span className="ml2">Processes</span>
                      </button>
                    </li>

                    <li className="mt3">
                      <button
                        className="flex items-center b--none bg-white pointer dim"
                        onClick={() => workspaceSend({type: "INVESTIGATION"})}
                      >
                        <img
                          height="20px"
                          width="20px"
                          src={investigationIcon}
                          alt="Go to workspace investigations."
                        />
                        <span className="ml2">Investigations</span>
                      </button>
                    </li>
                  </ul>
                </div>
                <div className="pa2 h3 w5 bt b--solitude bw1 flex items-center justify-between fixed bottom-0">
                  <button
                    className="b--none bg-white pointer dim"
                    onClick={() => ncubeSend("SHOW_DASHBOARD")}
                  >
                    <img height="45px" src={logoIcon} alt="Ncube logo." />
                  </button>
                  <img src={settingsIcon} alt="Go to settings." />
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </>
  );
};

export default Sidebar;
