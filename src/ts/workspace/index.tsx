import {useMachine} from "@xstate/react";
import React from "react";

import Fatal from "../common/fatal";
import {WorkspaceProvider} from "../context";
import Panel from "../layout/panel";
import machine from "../machines/workspace";
import {Workspace} from "../types";
import {useServiceLogger} from "../utils";
import SectionCard from "./section-card";
import Source from "./source";

interface WorkspaceProps {
  workspace: Workspace;
  workspaces: Workspace[];
}

const WorkspacePanel = ({workspaces, workspace}: WorkspaceProps) => {
  const [state, send, service] = useMachine(machine, {
    context: {
      workspace,
    },
  });

  useServiceLogger(service, machine.id);

  switch (true) {
    case state.matches("overview"):
      return (
        <WorkspaceProvider value={[state, send]}>
          <Panel
            workspaces={workspaces}
            workspace={workspace}
            header={workspace.name}
            description={workspace.description}
          >
            <div>
              <SectionCard onClick={() => send("SOURCE")} kind="source" />
              <SectionCard onClick={() => send("DATA")} kind="data" />
              <SectionCard onClick={() => send("PROCESS")} kind="process" />
              <SectionCard
                onClick={() => send("INVESTIGATION")}
                kind="investigation"
              />
            </div>
          </Panel>
        </WorkspaceProvider>
      );

    case state.matches("source"):
      return (
        <WorkspaceProvider value={[state, send]}>
          <Panel
            workspaces={workspaces}
            workspace={workspace}
            header="Sources"
            description=""
          >
            <Source workspace={workspace} />
          </Panel>
        </WorkspaceProvider>
      );

    case state.matches("data"):
      return (
        <WorkspaceProvider value={[state, send]}>
          <Panel
            workspaces={workspaces}
            workspace={workspace}
            header="Data"
            description=""
          >
            <div />
          </Panel>
        </WorkspaceProvider>
      );

    case state.matches("process"):
      return (
        <WorkspaceProvider value={[state, send]}>
          <Panel
            workspaces={workspaces}
            workspace={workspace}
            header="Processes"
            description=""
          >
            <div />
          </Panel>
        </WorkspaceProvider>
      );

    case state.matches("investigation"):
      return (
        <WorkspaceProvider value={[state, send]}>
          <Panel
            workspaces={workspaces}
            workspace={workspace}
            header="Investigations"
            description=""
          >
            <div />
          </Panel>
        </WorkspaceProvider>
      );

    default:
      return (
        <Fatal
          msg={`${machine.id} machine didn't match any valid state: ${state.value}`}
        />
      );
  }
};

export default WorkspacePanel;
