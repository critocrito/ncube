import {sourceType} from "@sugarcube/source-types";
import {assign, createMachine, DoneInvokeEvent} from "xstate";

import {currentUrl} from "../lib/discovery";
import {listWorkspaces, saveSource} from "../lib/handlers";
import {healthCheck} from "../lib/http";
import {SourceReq, Workspace} from "../types";

export type DiscoveryContext = {
  workspaces: Workspace[];
  sourceReq?: SourceReq;
  workspace?: Workspace;
  error?: string;
};

export type DiscoveryEvent =
  | {type: "PRESERVE"; url: string}
  | {type: "BACK"}
  | {type: "SELECT_WORKSPACE"; workspace: Workspace}
  | {type: "STORE_SOURCE"; sourceReq: SourceReq}
  | {type: "CLOSE"}
  | {type: "RETRY"};

export type DiscoveryState =
  | {value: "initial"; context: DiscoveryContext}
  | {
      value:
        | "healthy"
        | "workspaces"
        | "store_source"
        | "introduction"
        | "workspaces_select";
      context: DiscoveryContext & {sourceReq: SourceReq};
    }
  | {
      value: "source" | "success";
      context: DiscoveryContext & {sourceReq: SourceReq; workspace: Workspace};
    }
  | {value: "error"; context: DiscoveryContext & {error: string}};

export default createMachine<DiscoveryContext, DiscoveryEvent, DiscoveryState>(
  {
    id: "discovery",

    context: {
      workspaces: [],
    },

    initial: "initial",

    states: {
      initial: {
        invoke: {
          id: "initial",
          src: "parseSource",
          onDone: {
            target: "healthy",
            actions: "setSourceReq",
          },
          onError: {
            target: "error",
            actions: "fail",
          },
        },
      },

      healthy: {
        invoke: {
          id: "health",
          src: "healthCheck",
          onDone: "introduction",
          onError: {
            target: "error",
            actions: "fail",
          },
        },
      },

      workspaces: {
        invoke: {
          src: "listWorkspaces",

          onDone: {
            target: "workspaces_select",
            actions: "setWorkspaces",
          },

          onError: {
            target: "error",
            actions: "fail",
          },
        },
      },

      store_source: {
        invoke: {
          src: "storeSource",

          onDone: "success",

          onError: {
            target: "error",
            actions: "fail",
          },
        },
      },

      introduction: {
        on: {
          PRESERVE: "workspaces",
        },
      },

      workspaces_select: {
        on: {
          SELECT_WORKSPACE: {
            target: "source",
            actions: "setWorkspace",
          },

          CLOSE: {
            target: "introduction",
            actions: "closePopup",
          },
        },
      },

      source: {
        on: {
          STORE_SOURCE: {
            target: "store_source",
            actions: "setSourceReq",
          },

          CLOSE: {
            target: "introduction",
            actions: "closePopup",
          },
        },
      },

      success: {
        after: {
          3000: {
            actions: "closePopup",
          },
        },
      },

      error: {
        exit: "resetContext",

        on: {
          RETRY: "initial",
        },
      },
    },
  },
  {
    actions: {
      setSourceReq: assign({
        sourceReq: (_ctx, ev) => {
          const {data} = ev as DoneInvokeEvent<SourceReq>;
          return data;
        },
      }),

      setWorkspaces: assign({
        workspaces: (_ctx, ev) => {
          const {data} = ev as DoneInvokeEvent<Workspace[]>;
          return data;
        },
      }),

      setWorkspace: assign({
        workspace: (_ctx, ev) => {
          const {workspace} = ev as {
            type: "SELECT_WORKSPACE";
            workspace: Workspace;
          };
          return workspace;
        },
      }),

      closePopup: (): void => window.close(),

      resetContext: assign((ctx) => ({
        ...ctx,
        sourceReq: undefined,
        workspace: undefined,
        error: undefined,
      })),

      fail: assign({
        error: (_ctx, ev) => {
          const {data} = ev as DoneInvokeEvent<Error>;
          return data.message;
        },
      }),
    },

    services: {
      parseSource: async (): Promise<SourceReq> => {
        const url = await currentUrl();
        const source = sourceType(url);

        if (source === undefined)
          throw new Error(
            "The URL coudn't be recognized. This probably means that the URL is not valid or private.",
          );

        return {
          type: source,
          term: url,
          tags: [],
        };
      },

      healthCheck: async (ctx): Promise<void> => {
        try {
          await healthCheck();
        } catch {
          const {
            sourceReq: {term},
          } = ctx as DiscoveryContext & {
            sourceReq: SourceReq;
          };
          throw new Error(
            `Ncube is not running. Start Ncube to queue "${term}" for preservation.`,
          );
        }
      },

      listWorkspaces: (): Promise<Workspace[]> => listWorkspaces(),

      storeSource: async (ctx): Promise<void> => {
        const {sourceReq, workspace} = ctx as DiscoveryContext & {
          sourceReq: SourceReq;
          workspace: Workspace;
        };
        await saveSource(workspace.slug, sourceReq);
      },
    },
  },
);
