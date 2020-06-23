import {createMachine} from "xstate";

import {Workspace} from "../types";

export interface WorkspaceContext {
  workspace: Workspace;
}

export type WorkspaceEvent =
  | {type: "OVERVIEW"}
  | {type: "SOURCE"}
  | {type: "DATA"}
  | {type: "PROCESS"}
  | {type: "INVESTIGATION"};

export type WorkspaceState =
  | {
      value: "overview";
      context: WorkspaceContext;
    }
  | {
      value: "source" | "data" | "process" | "investigation";
      context: WorkspaceContext;
    };

export default createMachine<WorkspaceContext, WorkspaceEvent, WorkspaceState>({
  id: "workspace",
  initial: "overview",
  states: {
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
  },
});
