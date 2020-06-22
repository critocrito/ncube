import {assign, createMachine} from "xstate";

import {CreateWorkspaceFormValues} from "../forms/create-workspace";

export type CreateWorkspaceContext = {
  values?: CreateWorkspaceFormValues;
  error?: string;
};

export type CreateWorkspaceEvent =
  | {type: "SAVE"; values: CreateWorkspaceFormValues}
  | {type: "CANCEL"}
  | {type: "DONE"}
  | {type: "ERROR"; error: string; values: CreateWorkspaceFormValues}
  | {type: "RETRY"; values: CreateWorkspaceFormValues};

export type CreateWorkspaceState =
  | {
      value: "initial";
      context: CreateWorkspaceContext & {values: undefined; error: undefined};
    }
  | {value: "saving"; context: CreateWorkspaceContext}
  | {value: "done"; context: CreateWorkspaceContext}
  | {value: "error"; context: CreateWorkspaceContext & {error: string}};

export default createMachine<
  CreateWorkspaceContext,
  CreateWorkspaceEvent,
  CreateWorkspaceState
>({
  id: "createWorkspace",

  context: {
    values: undefined,
    error: undefined,
  },

  initial: "initial",

  states: {
    initial: {
      on: {
        SAVE: {
          target: "saving",
          actions: assign({values: (_ctx, ev) => ev.values}),
        },
        CANCEL: "done",
      },
    },

    saving: {
      invoke: {
        src: "store",
        onDone: {
          target: "done",
        },
        onError: {
          target: "error",
          actions: assign({error: (_ctx, ev) => ev.data}),
        },
      },
    },

    error: {
      on: {
        RETRY: "saving",
        CANCEL: "done",
      },
    },

    done: {
      type: "final",
    },
  },
});
