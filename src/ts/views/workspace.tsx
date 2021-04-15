import {useActor} from "@xstate/react";
import React from "react";

import Error from "../components/error";
import LoadingSpinner from "../components/loading-spinner";
import Panel from "../components/panel";
import Unreachable from "../components/unreachable";
import {WorkspaceProvider} from "../lib/context";
import machine, {WorkspaceMachineInterpreter} from "../machines/workspace";
import Data from "./data";
import Investigations from "./investigations";
import Processes from "./processes";
import Sources from "./sources";
import Workspaces from "./workspaces";

interface WorkspaceProps {
  workspaceRef: WorkspaceMachineInterpreter;
}

const WorkspacePanel = ({workspaceRef}: WorkspaceProps) => {
  const [state, send] = useActor(workspaceRef);

  if (state.matches("stats"))
    return (
      <div className="vh-100 w-100 flex flex-column justify-around items-center">
        <div>
          <LoadingSpinner />
        </div>
      </div>
    );

  if (state.matches("overview"))
    return (
      <WorkspaceProvider value={[state, send]}>
        <Workspaces />
      </WorkspaceProvider>
    );

  if (state.matches("source")) {
    return (
      <WorkspaceProvider value={[state, send]}>
        <Sources />
      </WorkspaceProvider>
    );
  }

  if (state.matches("data")) {
    return (
      <WorkspaceProvider value={[state, send]}>
        <Data />
      </WorkspaceProvider>
    );
  }

  if (state.matches("process"))
    return (
      <WorkspaceProvider value={[state, send]}>
        <Processes />
      </WorkspaceProvider>
    );

  if (state.matches("investigation"))
    return (
      <WorkspaceProvider value={[state, send]}>
        <Investigations />
      </WorkspaceProvider>
    );

  if (state.matches("error")) {
    const {error} = state.context;

    return (
      <WorkspaceProvider value={[state, send]}>
        <Panel header="">
          <Error
            msg={error || "Failed to fetch stats."}
            recover={() => send({type: "RETRY"})}
          />
        </Panel>
      </WorkspaceProvider>
    );
  }

  return <Unreachable machine={machine.id} state={state.value} />;
};

export default WorkspacePanel;
