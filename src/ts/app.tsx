import {useMachine} from "@xstate/react";
import React, {useEffect} from "react";

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

  useEffect(() => {
    const isCreating = workspaces.reduce((memo, {is_created: isCreated}) => {
      if (memo) return memo;
      return !isCreated;
    }, false);

    if (isCreating) {
      setTimeout(() => send("RELOAD_WORKSPACES"), 10 * 1000);
    }
  }, [send, workspaces]);

  switch (true) {
    case state.matches("onboarding"):
      return <Onboarding onDone={() => send("SHOW_HOME")} />;

    case state.matches("list_workspaces"):
    case state.matches("home"):
      return (
        <AppProvider value={[state, send]}>
          <Home onDone={() => {}} workspaces={workspaces} />
        </AppProvider>
      );

    case state.matches("show_workspace"):
      return <div />;

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
