import {assign, createMachine, DoneInvokeEvent} from "xstate";

import {Resource, SlugResource, Workspace} from "../types";

export type DeleteContext = {
  resource: Resource | SlugResource;
  workspace: Workspace;
  error?: string;
};

export type DeleteEventYes<T extends Record<string, unknown>> = {
  type: "YES";
  data: T;
};

export type DeleteEvent<T extends Record<string, unknown>> =
  | DeleteEventYes<T>
  | {type: "NO"}
  | {type: "RETRY"};

type DeleteState =
  | {value: "confirm" | "delete" | "abort" | "success"; context: DeleteContext}
  | {value: "error"; context: DeleteContext & {error: string}};

// eslint-disable-next-line @typescript-eslint/explicit-function-return-type
export default <T extends Record<string, unknown>>() =>
  createMachine<DeleteContext, DeleteEvent<T>, DeleteState>(
    {
      id: "delete-resource",
      initial: "confirm",
      states: {
        confirm: {
          on: {
            YES: "delete",
            NO: "abort",
          },
        },

        delete: {
          invoke: {
            src: "delete",
            onDone: "success",
            onError: {
              target: "error",
              actions: "fail",
            },
          },
        },

        error: {
          on: {
            RETRY: "confirm",
          },
        },

        success: {
          type: "final",
        },

        abort: {
          type: "final",
        },
      },
    },
    {
      actions: {
        fail: assign({
          error: (_ctx, ev) => {
            const {data} = ev as DoneInvokeEvent<Error>;
            return data.message;
          },
        }),
      },
    },
  );
