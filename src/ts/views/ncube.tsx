import React from "react";

import Error from "../components/error";
import Fatal from "../components/fatal";
import {useNcubeCtx} from "../lib/context";
import machine from "../machines/ncube";
import Dashboard from "./dashboard";
import Host from "./host";
import Workspace from "./workspace";

const Ncube = () => {
  const [state, send] = useNcubeCtx();

  if (state.matches("host")) {
    const {hostRef} = state.context;

    return <Host hostRef={hostRef} />;
  }

  if (state.matches("dashboard")) {
    const {dashboardRef} = state.context;

    return <Dashboard dashboardRef={dashboardRef} />;
  }

  if (state.matches("workspace")) {
    const {workspaceRef} = state.context;

    return <Workspace workspaceRef={workspaceRef} />;
  }

  if (state.matches("error")) {
    const {error} = state.context;

    return (
      <Error
        msg={error || "Failed to fetch workspaces."}
        recover={() => send("RETRY")}
      />
    );
  }

  return (
    <Fatal
      msg={`${machine.id} machine didn't match any valid state: ${state.value}`}
    />
  );
};

export default Ncube;
