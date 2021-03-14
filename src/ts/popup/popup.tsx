import {useMachine} from "@xstate/react";
import React from "react";

import Error from "../common/error";
import Fatal from "../common/fatal";
import {listWorkspaces, saveSource} from "../handlers";
import machine, {WebExtEventStoreSource} from "../machines/web-ext";
import {SourceReq, Workspace} from "../types";
import {useServiceLogger} from "../utils";
import Introduction from "./introduction";
import Source from "./source";
import Workspaces from "./workspaces";

interface PopupProps {
  sourceReq: SourceReq;
}

const Popup = ({sourceReq}: PopupProps) => {
  const [state, send, service] = useMachine(machine, {
    services: {
      listWorkspaces: (_ctx, _ev) => listWorkspaces(),

      storeSource: (_ctx, ev) => {
        const {source, workspace} = ev as WebExtEventStoreSource;
        return saveSource(workspace.slug, source);
      },
    },

    actions: {
      closePopup: (_ctx, _ev) => window.close(),
    },
  });

  useServiceLogger(service, machine.id);

  const {workspaces} = state.context;

  switch (true) {
    case state.matches("list_workspaces"):
    case state.matches("store_source"):
      return <div />;

    case state.matches("introduction"):
      return (
        <Introduction sourceReq={sourceReq} onNext={() => send("PRESERVE")} />
      );

    case state.matches("success"):
      return (
        <div className="mt4 pa4 bg-washed-green">
          <span className="b success">Success</span>
        </div>
      );

    case state.matches("workspaces"):
      return (
        <Workspaces
          onNext={(w: Workspace) => send("SELECT_WORKSPACE", {workspace: w})}
          workspaces={workspaces}
        />
      );

    case state.matches("source"):
      switch (state.event.type) {
        case "SELECT_WORKSPACE": {
          const {workspace} = state.event;
          return (
            <Source
              onNext={(source: SourceReq) =>
                send("STORE_SOURCE", {source, workspace})
              }
              onCancel={() => send("CLOSE")}
              workspace={workspace}
              sourceReq={sourceReq}
            />
          );
        }

        default:
          return (
            <Fatal
              msg={`${machine.id} machine didn't match any valid state: ${state.value}`}
            />
          );
      }

    case state.matches("error"):
      return (
        <Error
          msg={
            state.context.error ||
            "Failed to communicate with Ncube. Is it running?"
          }
          recover={() => send("RETRY")}
        />
      );

    default:
      return (
        <Fatal
          msg={`${machine.id} machine didn't match any valid state: ${state.value}`}
        />
      );
  }
};

export default Popup;
