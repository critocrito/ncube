import {assign, createMachine} from "xstate";

import {
  statDataSegments,
  statDataSources,
  statDataTotal,
  // statDataVideos,
  statSourcesTotal,
  statSourcesTypes,
} from "../lib/http";
import {DataStats, SourceStats, Workspace} from "../types";

export interface WorkspaceContext {
  workspace: Workspace;
  dataStats: DataStats;
  sourceStats: SourceStats;
  error?: string;
}

export type WorkspaceEvent =
  | {type: "OVERVIEW"}
  | {type: "SOURCE"}
  | {type: "DATA"}
  | {type: "PROCESS"}
  | {type: "INVESTIGATION"}
  | {type: "RETRY"}
  | {type: "CANCEL"};

export type WorkspaceState =
  | {
      value:
        | "stats"
        | "overview"
        | "source"
        | "data"
        | "process"
        | "investigation";
      context: WorkspaceContext;
    }
  | {
      value: "error";
      context: WorkspaceContext & {error: string};
    };

export default createMachine<WorkspaceContext, WorkspaceEvent, WorkspaceState>(
  {
    id: "workspace",
    initial: "stats",
    states: {
      stats: {
        invoke: {
          src: "fetchStats",

          onDone: {
            target: "overview",
            actions: assign({
              dataStats: (_, {data}) => data.dataStats,
              sourceStats: (_, {data}) => data.sourceStats,
            }),
          },

          onError: {
            target: "error",
            actions: assign({error: (_ctx, {data}) => data.message}),
          },
        },
      },

      overview: {
        on: {
          OVERVIEW: "overview",
          SOURCE: "source",
          DATA: "data",
          PROCESS: "process",
          INVESTIGATION: "investigation",
        },
      },

      source: {
        on: {
          OVERVIEW: "overview",
          SOURCE: "source",
          DATA: "data",
          PROCESS: "process",
          INVESTIGATION: "investigation",
        },
      },

      data: {
        on: {
          OVERVIEW: "overview",
          SOURCE: "source",
          DATA: "data",
          PROCESS: "process",
          INVESTIGATION: "investigation",
        },
      },

      process: {
        on: {
          OVERVIEW: "overview",
          SOURCE: "source",
          DATA: "data",
          PROCESS: "process",
          INVESTIGATION: "investigation",
        },
      },

      investigation: {
        on: {
          OVERVIEW: "overview",
          SOURCE: "source",
          DATA: "data",
          PROCESS: "process",
          INVESTIGATION: "investigation",
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
    services: {
      fetchStats: async (
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
