import {assign, createMachine} from "xstate";

import {Process, Workspace} from "../types";

type ProcessContext = {
  workspace: Workspace;
  processes: Process[];
  error?: string;
};

type ProcessEvent = {type: "RETRY"};

type ProcessState =
  | {
      value: "processes" | "home";
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

    home: {
      on: {},
    },

    error: {
      on: {
        RETRY: "home",
      },
    },
  },
});
