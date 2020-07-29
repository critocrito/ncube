import {assign, createMachine} from "xstate";

import {Process, ProcessConfigReq, Workspace} from "../types";

type ProcessContext = {
  workspace: Workspace;
  processes: Process[];
  error?: string;
};

type ProcessEvent =
  | {type: "SHOW_DETAILS"; process: Process}
  | {type: "RUN"; process: Process}
  | {type: "SAVE_CONFIG"; config: ProcessConfigReq}
  | {type: "SHOW_HOME"}
  | {type: "RETRY"};

type ProcessState =
  | {
      value: "processes" | "configure" | "run" | "home" | "details";
      context: ProcessContext;
    }
  | {
      value: "error";
      context: ProcessContext & {error: string};
    };

export default createMachine<ProcessContext, ProcessEvent, ProcessState>({
  id: "process",
  initial: "processes",
  states: {
    processes: {
      invoke: {
        src: "fetchProcesses",

        onDone: {
          target: "home",
          actions: assign({processes: (_, {data}) => data}),
        },

        onError: {
          target: "error",
          actions: assign({error: (_ctx, {data}) => data.message}),
        },
      },
    },

    configure: {
      invoke: {
        src: "storeProcessConfig",

        onDone: {
          target: "processes",
        },

        onError: {
          target: "error",
          actions: assign({error: (_ctx, {data}) => data.message}),
        },
      },
    },

    run: {
      invoke: {
        src: "runProcess",

        onDone: {
          target: "processes",
        },

        onError: {
          target: "error",
          actions: assign({error: (_ctx, {data}) => data.message}),
        },
      },
    },

    home: {
      on: {
        SHOW_DETAILS: "details",
        RUN: "run",
      },
    },

    details: {
      on: {
        SHOW_HOME: "home",
        SAVE_CONFIG: "configure",
      },
    },

    error: {
      on: {
        RETRY: "home",
      },
    },
  },
});
