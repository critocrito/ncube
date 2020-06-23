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
  | {type: "RETRY"};

type SourceState =
  | {
      value: "listing";
      context: SourceContext;
    }
  | {
      value: "home" | "create";
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
        src: "listSources",

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

    home: {
      on: {
        CREATE_SOURCE: "create",
      },
    },

    create: {
      on: {
        SHOW_HOME: "listing",
      },
    },

    error: {
      on: {
        RETRY: "listing",
      },
    },
  },
});
