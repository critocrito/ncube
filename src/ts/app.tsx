import {useMachine} from "@xstate/react";
import React from "react";

import Error from "./common/error";
import Fatal from "./common/fatal";
import {AppProvider} from "./context";
import {listWorkspaces} from "./handlers";
import Home from "./home";
import {showWorkspace} from "./http";
import machine from "./machines/app";
import Onboarding from "./onboarding";
import {useServiceLogger} from "./utils";
import Workspace from "./workspace";

const App = () => {
  const [state, send, service] = useMachine(machine, {
    services: {
      listWorkspaces: async (_ctx, _ev) => listWorkspaces(),

      fetchWorkspace: async (_ctx, {slug}) => {
        return showWorkspace(slug);
      },
    },
  });

  useServiceLogger(service, machine.id);

  const {workspace, workspaces} = state.context;

  switch (true) {
    case state.matches("onboarding"):
      return <Onboarding onDone={() => send("SHOW_HOME")} />;

    case state.matches("listing"):
      return <div />;

    case state.matches("home"):
      return (
        <AppProvider value={[state, send]}>
          <Home onDone={() => {}} workspaces={workspaces} />
        </AppProvider>
      );

    case state.matches("fetchWorkspace") && workspace === undefined:
      return <div />;

    case state.matches("fetchWorkspace") && workspace:
    case state.matches("workspace"):
      if (workspace === undefined)
        return (
          <Fatal
            msg="The workspace state lacks a workspace."
            reset={() => send("RESTART_APP")}
          />
        );

      return (
        <AppProvider value={[state, send]}>
          <Workspace workspaces={workspaces} workspace={workspace} />
        </AppProvider>
      );

    case state.matches("error"):
      return (
        <Error
          msg={state.context.error || "Failed to fetch workspaces."}
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

export default App;
