import {assign, createMachine} from "xstate";

import PubSub from "../lib/pubsub";
import {Workspace} from "../types";

export interface AppContext {
  workspaces: Workspace[];
  pubsub: PubSub;
  ws?: WebSocket;
  workspace?: Workspace;
  error?: string;
}

export type AppEventShowWorkspace = {
  type: "SHOW_WORKSPACE";
  workspace: Workspace;
};

export type AppEventReallyDeleteWorkspace = {
  type: "REALLY_DELETE_WORKSPACE";
  workspace: Workspace;
  removeLocation: boolean;
};

export type AppEvent =
  | {type: "SHOW_DASHBOARD"; ws: WebSocket}
  | AppEventShowWorkspace
  | {type: "CREATE_WORKSPACE"}
  | {type: "LINK_WORKSPACE"}
  | {
      type: "DELETE_WORKSPACE";
      workspace: Workspace;
    }
  | AppEventReallyDeleteWorkspace
  | {type: "RELOAD_WORKSPACES"}
  | {type: "RESTART_APP"}
  | {type: "RETRY"};

export type AppState =
  | {
      value:
        | "onboarding"
        | "list_workspaces"
        | "show_workspace"
        | "delete_workspace"
        | "dashboard"
        | "create"
        | "link";
      context: AppContext;
    }
  | {
      value: "error";
      context: AppContext & {error: string};
    }
  | {
      value: "workspace" | "confirm_delete";
      context: AppContext & {workspace: Workspace};
    };

export default createMachine<AppContext, AppEvent, AppState>(
  {
    id: "app",

    initial: "onboarding",

    states: {
      onboarding: {
        on: {
          SHOW_DASHBOARD: {
            target: "list_workspaces",
            actions: assign({ws: (_ctx, {ws}) => ws}),
          },
        },
      },

      list_workspaces: {
        invoke: {
          src: "listWorkspaces",

          onDone: {
            target: "dashboard",
            actions: assign({workspaces: (_ctx, {data}) => data}),
          },

          onError: {
            target: "error",
            actions: assign({error: (_ctx, {data}) => data.message}),
          },
        },
      },

      show_workspace: {
        invoke: {
          src: "fetchWorkspace",

          onDone: {
            target: "workspace",
            actions: assign({workspace: (_ctx, {data}) => data}),
          },

          onError: {
            target: "error",
            actions: assign({error: (_ctx, {data}) => data.message}),
          },
        },
      },

      delete_workspace: {
        invoke: {
          src: "deleteWorkspace",

          onDone: {
            target: "dashboard",
            actions: assign(({workspaces, ...ctx}, {data}) => ({
              ...ctx,
              workspaces: workspaces.filter(({slug}) => slug !== data.slug),
              workspace: undefined,
            })),
          },

          onError: {
            target: "error",
            actions: assign({error: (_ctx, {data}) => data.message}),
          },
        },
      },

      dashboard: {
        entry: ["resetContext"],
        on: {
          SHOW_WORKSPACE: "show_workspace",
          RESTART_APP: "onboarding",
          RELOAD_WORKSPACES: "list_workspaces",
          CREATE_WORKSPACE: "create",
          LINK_WORKSPACE: "link",
          DELETE_WORKSPACE: {
            target: "confirm_delete",
            actions: assign({workspace: (_ctx, {workspace}) => workspace}),
          },
        },
      },

      create: {
        on: {
          SHOW_DASHBOARD: "dashboard",
          RELOAD_WORKSPACES: "list_workspaces",
        },
      },

      link: {
        on: {
          SHOW_DASHBOARD: "dashboard",
          RELOAD_WORKSPACES: "list_workspaces",
        },
      },

      confirm_delete: {
        on: {
          SHOW_DASHBOARD: "dashboard",
          REALLY_DELETE_WORKSPACE: "delete_workspace",
        },
      },

      workspace: {
        on: {
          SHOW_WORKSPACE: "show_workspace",
          RESTART_APP: "onboarding",
          SHOW_DASHBOARD: "dashboard",
        },
      },

      error: {
        on: {
          RETRY: "list_workspaces",
        },
      },
    },
  },
  {
    actions: {
      resetContext: assign((ctx) => ({
        ...ctx,
        workspace: undefined,
        error: undefined,
      })),
    },
  },
);
