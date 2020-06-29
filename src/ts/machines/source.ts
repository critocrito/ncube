import {assign, createMachine} from "xstate";

import {Source, Workspace} from "../types";

type SourceContext = {
  workspace: Workspace;
  sources?: Source[];
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
      value: "listing" | "deleting";
      context: SourceContext;
    }
  | {
      value: "home" | "create" | "delete";
      context: SourceContext & {sources: Source[]};
    }
  | {
      value: "error";
      context: SourceContext & {error: string};
    };

export default createMachine<SourceContext, SourceEvent, SourceState>({
  id: "source",
  initial: "listing",
  states: {
    listing: {
      invoke: {
        src: "fetchData",

        onDone: {
          target: "home",
          actions: assign({
            sources: (_, {data}) => data,
          }),
        },

        onError: {
          target: "error",
          actions: assign({error: (_ctx, {data}) => data.message}),
        },
      },
    },

    deleting: {
      invoke: {
        src: "deleteSource",

        onDone: {
          target: "listing",
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
        SHOW_HOME: "listing",
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
        RETRY: "listing",
      },
    },
  },
});
