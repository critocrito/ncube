import {useMachine} from "@xstate/react";
import React, {useState} from "react";

import DataIntroduction from "../../mdx/data-intro.mdx";
import InvestigationsIntroduction from "../../mdx/investigations-intro.mdx";
import ProcessesIntroduction from "../../mdx/processes-intro.mdx";
import SourcesIntroduction from "../../mdx/sources-intro.mdx";
import Error from "../common/error";
import Fatal from "../common/fatal";
import IntroText from "../common/intro-text";
import LoadingSpinner from "../common/loading-spinner";
import Panel from "../common/panel";
import {WorkspaceProvider} from "../context";
import {
  statDataSegments,
  statDataSources,
  statDataTotal,
  // statDataVideos,
  statSourcesTotal,
  statSourcesTypes,
} from "../http";
import Investigation from "../investigation";
import machine from "../machines/workspace";
import Process from "../process";
import {Workspace} from "../types";
import {useServiceLogger} from "../utils";
import Database from "./database";
import SectionCard from "./section-card";
import SourcesTable from "./sources-table";

interface WorkspaceProps {
  workspace: Workspace;
  workspaces: Workspace[];
}

const WorkspacePanel = ({workspaces, workspace}: WorkspaceProps) => {
  const [databaseHeader, setDatabaseHeader] = useState("Database");
  const [investigationsHeader, setInvestigationsHeader] = useState(
    "Investigations",
  );

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

    services: {
      fetchStats: async (_ctx, _ev) => {
        const [
          dataTotal,
          dataSources,
          dataSegments,
          // dataVideos,
          sourcesTotal,
          sourcesTypes,
        ] = await Promise.all([
          statDataTotal(workspace.slug),
          statDataSources(workspace.slug),
          statDataSegments(workspace.slug),
          // statDataVideos(workspace.slug),
          statSourcesTotal(workspace.slug),
          statSourcesTypes(workspace.slug),
        ]);

        return {
          dataStats: {
            total: dataTotal,
            sources: dataSources,
            segments: dataSegments,
            // videos: dataVideos,
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
          <Panel
            workspaces={workspaces}
            workspace={workspace}
            header={workspace.name}
            description={workspace.description}
          >
            <>
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
            </>
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
            <>
              <IntroText>
                <SourcesIntroduction />
              </IntroText>

              <SourcesTable
                workspace={workspace}
                totalStat={sourceStats.total}
              />
            </>
          </Panel>
        </WorkspaceProvider>
      );

    case state.matches("data"):
      return (
        <WorkspaceProvider value={[state, send]}>
          <Panel
            workspaces={workspaces}
            workspace={workspace}
            header={databaseHeader}
            description=""
          >
            <>
              <IntroText>
                <DataIntroduction />
              </IntroText>

              <Database
                stats={dataStats}
                workspace={workspace}
                onHeaderChange={(title) =>
                  setDatabaseHeader(title ? `Database: ${title}` : "Database")
                }
              />
            </>
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
            <>
              <IntroText>
                <ProcessesIntroduction />
              </IntroText>

              <Process workspace={workspace} />
            </>
          </Panel>
        </WorkspaceProvider>
      );

    case state.matches("investigation"):
      return (
        <WorkspaceProvider value={[state, send]}>
          <Panel
            workspaces={workspaces}
            workspace={workspace}
            header={investigationsHeader}
            description=""
          >
            <>
              <IntroText>
                <InvestigationsIntroduction />
              </IntroText>

              <Investigation
                workspace={workspace}
                onHeaderChange={(title) =>
                  setInvestigationsHeader(
                    title ? `Investigations: ${title}` : "Investigations",
                  )
                }
              />
            </>
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
