import {assign, createMachine} from "xstate";

import {SourceReq, Workspace} from "../types";

export interface WebExtContext {
  workspaces: Workspace[];
  error?: string;
}

export type WebExtEvent =
  | {type: "PRESERVE"; url: string}
  | {type: "BACK"}
  | {type: "SELECT_WORKSPACE"; workspace: Workspace}
  | {type: "STORE_SOURCE"; source: SourceReq; workspace: Workspace}
  | {type: "CLOSE"}
  | {type: "RETRY"};

export type WebExtState =
  | {
      value:
        | "list_workspaces"
        | "store_source"
        | "introduction"
        | "workspaces"
        | "source"
        | "success";
      context: WebExtContext;
    }
  | {value: "error"; context: WebExtContext & {error: string}};

export default createMachine<WebExtContext, WebExtEvent, WebExtState>({
  id: "web-ext",

  context: {
    workspaces: [],
  },

  initial: "introduction",

  states: {
    list_workspaces: {
      invoke: {
        src: "listWorkspaces",

        onDone: {
          target: "workspaces",
          actions: assign({workspaces: (_ctx, {data}) => data}),
        },

        onError: {
          target: "error",
          actions: assign({error: (_ctx, {data}) => data.message}),
        },
      },
    },

    store_source: {
      invoke: {
        src: "storeSource",

        onDone: {
          target: "success",
        },

        onError: {
          target: "error",
          actions: assign({error: (_ctx, {data}) => data.message}),
        },
      },
    },

    introduction: {
      on: {
        PRESERVE: "list_workspaces",
      },
    },

    workspaces: {
      on: {
        SELECT_WORKSPACE: "source",

        CLOSE: {
          target: "introduction",
          actions: ["closePopup"],
        },
      },
    },

    source: {
      on: {
        STORE_SOURCE: "store_source",

        CLOSE: {
          target: "introduction",
          actions: ["closePopup"],
        },
      },
    },

    success: {
      after: {
        3000: {
          actions: ["closePopup"],
        },
      },
    },

    error: {
      on: {
        RETRY: "introduction",
      },
    },
  },
});
