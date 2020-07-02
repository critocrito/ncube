import {useMachine} from "@xstate/react";
import React from "react";

import Error from "../common/error";
import Fatal from "../common/fatal";
import Panel from "../common/panel";
import {WorkspaceProvider} from "../context";
import {
  statDataSources,
  statDataTotal,
  statDataVideos,
  statSourcesTotal,
  statSourcesTypes,
} from "../http";
import machine from "../machines/workspace";
import {Workspace} from "../types";
import {useServiceLogger} from "../utils";
import Database from "./database";
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
      dataStats: {
        total: 0,
        sources: 0,
        videos: 0,
      },
      sourceStats: {
        total: 0,
        types: 0,
      },
    },

    services: {
      fetchStats: async (_ctx, _ev) => {
        const [
          dataTotal,
          dataSources,
          dataVideos,
          sourcesTotal,
          sourcesTypes,
        ] = await Promise.all([
          statDataTotal(workspace.slug),
          statDataSources(workspace.slug),
          statDataVideos(workspace.slug),
          statSourcesTotal(workspace.slug),
          statSourcesTypes(workspace.slug),
        ]);

        return {
          dataStats: {
            total: dataTotal,
            sources: dataSources,
            videos: dataVideos,
          },
          sourceStats: {
            total: sourcesTotal,
            types: sourcesTypes,
          },
        };
      },
    },
  });

  useServiceLogger(service, machine.id);

  const {dataStats, sourceStats, error} = state.context;

  switch (true) {
    case state.matches("stats"):
      return <div />;

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
              <SectionCard
                onClick={() => send("SOURCE")}
                kind="source"
                stats={sourceStats}
              />
              <SectionCard
                onClick={() => send("DATA")}
                kind="data"
                stats={dataStats}
              />
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
            <Source workspace={workspace} stats={sourceStats} />
          </Panel>
        </WorkspaceProvider>
      );

    case state.matches("data"):
      return (
        <WorkspaceProvider value={[state, send]}>
          <Panel
            workspaces={workspaces}
            workspace={workspace}
            header="Database"
            description=""
          >
            <Database stats={dataStats} workspace={workspace} />
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

    case state.matches("error"):
      return (
        <WorkspaceProvider value={[state, send]}>
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
