import {assign, createMachine} from "xstate";

import {Source, Workspace} from "../types";

type SourceContext = {
  workspace: Workspace;
  error?: string;
};

type SourceEvent =
  | {type: "SHOW_HOME"}
  | {type: "CREATE_SOURCE"}
  | {type: "DELETE_SOURCE"; source: Source}
  | {type: "RETRY"}
  | {type: "CANCEL"}
  | {type: "CONFIRM_DELETE"; sourceId: number};

type SourceState =
  | {
      value: "home" | "create" | "delete" | "deleting";
      context: SourceContext;
    }
  | {
      value: "error";
      context: SourceContext & {error: string};
    };

export default createMachine<SourceContext, SourceEvent, SourceState>({
  id: "source",
  initial: "home",
  states: {
    deleting: {
      invoke: {
        src: "deleteSource",

        onDone: {
          target: "home",
        },

        onError: {
          target: "error",
          actions: assign({error: (_ctx, {data}) => data.message}),
        },
      },
    },

    home: {
      on: {
        CREATE_SOURCE: "create",
        DELETE_SOURCE: "delete",
      },
    },

    create: {
      on: {
        SHOW_HOME: "home",
      },
    },

    delete: {
      on: {
        CANCEL: "home",
        CONFIRM_DELETE: "deleting",
      },
    },

    error: {
      on: {
        RETRY: "home",
      },
    },
  },
});
