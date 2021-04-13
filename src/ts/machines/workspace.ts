import {
  ActorRefFrom,
  assign,
  createMachine,
  DoneInvokeEvent,
  Interpreter,
  send,
} from "xstate";

import {
  statDataSegments,
  statDataSources,
  statDataTotal,
  // statDataVideos,
  statSourcesTotal,
  statSourcesTypes,
} from "../lib/http";
import {spawn} from "../lib/utils";
import {DataStats, SourceStats, Workspace} from "../types";
import databaseMachine, {DatabaseMachineInterpreter} from "./database";
import investigationMachine, {
  InvestigationMachineInterpreter,
} from "./investigation";
import processMachine, {ProcessMachineInterpreter} from "./process";
import sourceMachine, {SourceMachineInterpreter} from "./source";

export interface WorkspaceContext {
  workspace: Workspace;
  dataStats: DataStats;
  sourceStats: SourceStats;
  header: string;
  sourcesRef?: SourceMachineInterpreter; // TableMachineInterpreter;
  databaseRef?: DatabaseMachineInterpreter;
  processRef?: ProcessMachineInterpreter;
  investigationRef?: InvestigationMachineInterpreter;
  error?: string;
}

type WorkspaceEventHeader = {type: "HEADER"; header: string};

export type WorkspaceEvent =
  | {type: "OVERVIEW"}
  | {type: "SOURCE"}
  | {type: "DATA"}
  | {type: "PROCESS"}
  | {type: "INVESTIGATION"}
  | {type: "RETRY"}
  | {type: "CANCEL"}
  | WorkspaceEventHeader;

export type WorkspaceState =
  | {
      value: "stats" | "overview";
      context: WorkspaceContext;
    }
  | {
      value: "source";
      context: WorkspaceContext & {sourcesRef: SourceMachineInterpreter};
    }
  | {
      value: "data";
      context: WorkspaceContext & {databaseRef: DatabaseMachineInterpreter};
    }
  | {
      value: "process";
      context: WorkspaceContext & {processRef: ProcessMachineInterpreter};
    }
  | {
      value: "investigation";
      context: WorkspaceContext & {
        investigationRef: InvestigationMachineInterpreter;
      };
    }
  | {
      value: "error";
      context: WorkspaceContext & {error: string};
    };

export type WorkspaceMachineInterpreter = ActorRefFrom<
  Interpreter<WorkspaceContext, WorkspaceState, WorkspaceEvent>["machine"]
>;

export default createMachine<WorkspaceContext, WorkspaceEvent, WorkspaceState>(
  {
    id: "workspace",

    initial: "stats",

    states: {
      stats: {
        invoke: {
          id: "stats",
          src: "showStats",
          onDone: {
            target: "overview",
            actions: "setStats",
          },
          onError: {
            target: "error",
            actions: "fail",
          },
        },
      },

      overview: {
        entry: "setOverviewHeader",

        on: {
          OVERVIEW: "overview",
          SOURCE: "source",
          DATA: "data",
          PROCESS: "process",
          INVESTIGATION: "investigation",
        },
      },

      source: {
        entry: ["spawnSources", "setSourceHeader"],

        on: {
          OVERVIEW: "overview",
          SOURCE: "source",
          DATA: "data",
          PROCESS: "process",
          INVESTIGATION: "investigation",
        },
      },

      data: {
        entry: ["spawnDatabase", "setDatabaseHeader"],

        exit: "resetData",

        on: {
          OVERVIEW: "overview",
          SOURCE: "source",
          DATA: "data",
          PROCESS: "process",
          INVESTIGATION: "investigation",
          HEADER: {
            actions: "setHeader",
          },
        },
      },

      process: {
        entry: ["spawnProcess", "setProcessHeader"],

        on: {
          OVERVIEW: "overview",
          SOURCE: "source",
          DATA: "data",
          PROCESS: "process",
          INVESTIGATION: "investigation",
        },
      },

      investigation: {
        entry: ["spawnInvestigation", "setInvestigationHeader"],

        exit: "resetInvestigation",

        on: {
          OVERVIEW: "overview",
          SOURCE: "source",
          DATA: "data",
          PROCESS: "process",
          INVESTIGATION: "investigation",
          HEADER: {
            actions: "setHeader",
          },
        },
      },

      error: {
        on: {
          RETRY: "stats",
        },
      },
    },
  },
  {
    actions: {
      spawnSources: assign<WorkspaceContext>({
        sourcesRef: ({workspace, sourcesRef, sourceStats}) =>
          sourcesRef ||
          spawn(
            sourceMachine.withContext({
              workspace,
              sourceStats,
              sources: [],
              selectedSources: [],
              total: sourceStats.total,
            }),
            "source",
          ),
      }),

      spawnDatabase: assign<WorkspaceContext>({
        databaseRef: ({workspace, dataStats, databaseRef}) =>
          databaseRef ||
          spawn(
            databaseMachine.withContext({
              workspace,
              dataStats,
              units: [],
              selectedUnits: [],
              segments: [],
              searchQuery: "",
              total: dataStats.total,
            }),
            "database",
          ),
      }),

      spawnProcess: assign<WorkspaceContext>({
        processRef: ({workspace, processRef}) =>
          processRef ||
          spawn(
            processMachine.withContext({workspace, processes: []}),
            "process",
          ),
      }),

      spawnInvestigation: assign<WorkspaceContext>({
        investigationRef: ({workspace, investigationRef}) =>
          investigationRef ||
          spawn(
            investigationMachine.withContext({
              workspace,
              investigations: [],
              segments: [],
            }),
            "investigation",
          ),
      }),

      setOverviewHeader: assign({
        header: ({workspace}) => `${workspace.name}`,
      }),

      setSourceHeader: assign({header: () => "Sources"}),

      setDatabaseHeader: assign({header: () => "Database"}),

      setProcessHeader: assign({header: () => "Processes"}),

      setInvestigationHeader: assign({header: () => "Investigations"}),

      setHeader: assign({
        header: (_ctx, ev) => {
          const {header} = ev as WorkspaceEventHeader;

          return header;
        },
      }),

      setStats: assign((ctx, ev) => {
        const {
          data: {dataStats, sourceStats},
        } = ev as DoneInvokeEvent<{
          dataStats: DataStats;
          sourceStats: SourceStats;
        }>;

        return {
          ...ctx,
          dataStats,
          sourceStats,
        };
      }),

      resetData: send("SHOW_HOME", {to: "database"}),

      resetInvestigation: send("SHOW_HOME", {to: "investigation"}),

      fail: assign({
        error: (_ctx, ev) => {
          const {data} = ev as DoneInvokeEvent<Error>;
          return data.message;
        },
      }),
    },

    services: {
      showStats: async (
        {workspace},
        _ev,
      ): Promise<{dataStats: DataStats; sourceStats: SourceStats}> => {
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
  },
);
