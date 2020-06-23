import c from "classnames";
import React, {useState} from "react";

import dataIcon from "../../../resources/public/images/icon_data.svg";
import investigationIcon from "../../../resources/public/images/icon_investigation.svg";
import processIcon from "../../../resources/public/images/icon_process.svg";
import sourceIcon from "../../../resources/public/images/icon_source.svg";
import logoIcon from "../../../resources/public/images/logo_horizontal.svg";
import settingsIcon from "../../../resources/public/images/settings.svg";
import {useAppCtx, useWorkspaceCtx} from "../context";
import {Workspace} from "../types";

interface SidebarProps {
  workspaces: Workspace[];
}

const Sidebar = ({workspaces}: SidebarProps) => {
  const [expanded, setExpanded] = useState(true);
  const [, appSend] = useAppCtx();
  const [workspaceState, workspaceSend] = useWorkspaceCtx();

  const toggleExpanded = () => setExpanded(!expanded);

  const buttonLabel = expanded ? "<" : ">";

  return (
    <>
      <div
        className="absolute cursor"
        style={{
          top: "50%",
          left: expanded ? "238px" : "0px",
        }}
      >
        <button
          className="sapphire b h2 w2 br-100 shadow-1 b--none bg-canvas flex justify-around items-center"
          onClick={toggleExpanded}
        >
          <span>{buttonLabel}</span>
        </button>
      </div>
      <div className={c(!expanded ? "dn" : undefined)}>
        <div className="w5 vh-100 flex flex-column br b--solitude">
          <div className="pa2 bg-sapphire h4">
            <select
              defaultValue={workspaceState.context.workspace.slug}
              className="w-100 mt5 b white b--sapphire bg-sapphire"
              onChange={(ev) => {
                appSend("SHOW_WORKSPACE", {slug: ev.target.value});
                workspaceSend("OVERVIEW");
              }}
            >
              {workspaces.map(({name, slug}) => (
                <option key={slug} value={slug} label={name} />
              ))}
            </select>
          </div>

          <div className="flex flex-column justify-between h-100">
            <div className="pa2">
              <ul className="list pl0">
                <li className="flex items-center">
                  <button
                    className="b--none bg-canvas"
                    onClick={() => workspaceSend("SOURCE")}
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

                <li className="flex items-center mt3">
                  <button
                    className="b--none bg-canvas"
                    onClick={() => workspaceSend("DATA")}
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

                <li className="flex items-center mt3">
                  <button
                    className="b--none bg-canvas"
                    onClick={() => workspaceSend("PROCESS")}
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

                <li className="flex items-center mt3">
                  <button
                    className="b--none bg-canvas"
                    onClick={() => workspaceSend("INVESTIGATION")}
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
            <div className="pa2 h3 w5 bt b--solitude bw2 flex items-center justify-between">
              <button
                className="b--none bg-canvas"
                onClick={() => appSend("SHOW_HOME")}
              >
                <img src={logoIcon} alt="Ncube logo." />
              </button>
              <img src={settingsIcon} alt="Go to settings." />
            </div>
          </div>
        </div>
      </div>
    </>
  );
};

export default Sidebar;