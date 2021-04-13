import {
  ActorRefFrom,
  assign,
  createMachine,
  DoneInvokeEvent,
  Interpreter,
} from "xstate";

import {listProcesses, runProcess, updateProcessConfig} from "../lib/http";
import {Process, ProcessConfigReq, Workspace} from "../types";

type ProcessContext = {
  workspace: Workspace;
  processes: Process[];
  error?: string;
};

export type ProcessEventRun = {type: "RUN"; process: Process};
export type ProcessEventSaveConfig = {
  type: "STORE_CONFIG";
  config: ProcessConfigReq;
};

type ProcessEvent =
  | {type: "SHOW_DETAILS"; process: Process}
  | ProcessEventRun
  | ProcessEventSaveConfig
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

export type ProcessMachineInterpreter = ActorRefFrom<
  Interpreter<ProcessContext, ProcessState, ProcessEvent>["machine"]
>;

export default createMachine<ProcessContext, ProcessEvent, ProcessState>(
  {
    id: "process",
    initial: "processes",
    states: {
      processes: {
        invoke: {
          src: "fetchProcesses",
          onDone: {
            target: "home",
            actions: "setProcesses",
          },
          onError: {
            target: "error",
            actions: "fail",
          },
        },
      },

      configure: {
        invoke: {
          src: "storeConfig",
          onDone: "processes",
          onError: {
            target: "error",
            actions: "fail",
          },
        },
      },

      run: {
        invoke: {
          src: "runProcess",
          onDone: "processes",
          onError: {
            target: "error",
            actions: "fail",
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
          STORE_CONFIG: "configure",
        },
      },

      error: {
        on: {
          RETRY: "home",
        },
      },
    },
  },
  {
    actions: {
      setProcesses: assign({
        processes: (_ctx, ev) => {
          const {data} = ev as DoneInvokeEvent<Process[]>;
          return data;
        },
      }),

      fail: assign({
        error: (_ctx, ev) => {
          const {data} = ev as DoneInvokeEvent<Error>;
          return data.message;
        },
      }),
    },

    services: {
      fetchProcesses: ({workspace}): Promise<Process[]> =>
        listProcesses(workspace.slug),

      storeConfig: ({workspace}, ev): Promise<void> => {
        const {config} = ev as ProcessEventSaveConfig;
        return updateProcessConfig(workspace.slug, config);
      },

      runProcess: ({workspace}, ev): Promise<void> => {
        const {
          process: {key},
        } = ev as ProcessEventRun;
        return runProcess(workspace.slug, {key, kind: "all"});
      },
    },
  },
);
