import {useMachine} from "@xstate/react";
import React from "react";

import Error from "../components/error";
import LoadingSpinner from "../components/loading-spinner";
import Panel from "../components/panel";
import Unreachable from "../components/unreachable";
import WorkspaceOverview from "../components/workspace/overview";
import {WorkspaceProvider} from "../lib/context";
import {useServiceLogger} from "../lib/utils";
import machine from "../machines/workspace";
import {Workspace} from "../types";
import Data from "./data";
import Investigations from "./investigations";
import Processes from "./processes";
import Sources from "./sources";

interface WorkspaceProps {
  workspace: Workspace;
  workspaces: Workspace[];
}

const WorkspacePanel = ({workspaces, workspace}: WorkspaceProps) => {
  const [state, send, service] = useMachine(machine, {
    context: {
      workspace,
      dataStats: {
        total: 0,
        sources: 0,
        segments: 0,
        // videos: 0,
      },
      sourceStats: {
        total: 0,
        types: 0,
      },
    },
  });

  useServiceLogger(service, machine.id);

  const {error} = state.context;

  switch (true) {
    case state.matches("stats"):
      return (
        <div className="vh-100 w-100 flex flex-column justify-around items-center">
          <div>
            <LoadingSpinner />
          </div>
        </div>
      );

    case state.matches("overview"):
      return (
        <WorkspaceProvider value={[state, send]}>
          <WorkspaceOverview />
        </WorkspaceProvider>
      );

    case state.matches("source"):
      return (
        <WorkspaceProvider value={[state, send]}>
          <Sources />
        </WorkspaceProvider>
      );

    case state.matches("data"):
      return (
        <WorkspaceProvider value={[state, send]}>
          <Data />
        </WorkspaceProvider>
      );

    case state.matches("process"):
      return (
        <WorkspaceProvider value={[state, send]}>
          <Processes />
        </WorkspaceProvider>
      );

    case state.matches("investigation"):
      return (
        <WorkspaceProvider value={[state, send]}>
          <Investigations />
        </WorkspaceProvider>
      );

    case state.matches("error"):
      return (
        <Panel
          workspaces={workspaces}
          workspace={workspace}
          header=""
          description=""
        >
          <Error
            msg={error || "Failed to fetch stats."}
            recover={() => send("RETRY")}
          />
        </Panel>
      );

    default:
      return <Unreachable machine={machine.id} state={state.value} />;
  }
};

export default WorkspacePanel;
