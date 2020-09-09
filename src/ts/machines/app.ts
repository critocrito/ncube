import {assign, createMachine} from "xstate";

import {Workspace} from "../types";

export interface AppContext {
  workspaces: Workspace[];
  error?: string;
}

export type AppEvent =
  | {type: "SHOW_DASHBOARD"}
  | {type: "SHOW_WORKSPACE"; workspace: Workspace}
  | {type: "done.invoke.fetchWorkspace"; data: Workspace}
  | {type: "CREATE_WORKSPACE"}
  | {type: "LINK_WORKSPACE"}
  | {type: "DELETE_WORKSPACE"; workspace: Workspace}
  | {
      type: "REALLY_DELETE_WORKSPACE";
      workspace: Workspace;
      removeLocation: boolean;
    }
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
        | "workspace"
        | "dashboard"
        | "create"
        | "link"
        | "confirm_delete";
      context: AppContext;
    }
  | {
      value: "error";
      context: AppContext & {error: string};
    }
  | {
      value: "workspace";
      context: AppContext & {workspace: Workspace};
    };

export default createMachine<AppContext, AppEvent, AppState>({
  id: "app",

  context: {
    workspaces: [],
  },

  initial: "onboarding",

  states: {
    onboarding: {
      on: {
        SHOW_DASHBOARD: "list_workspaces",
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
          actions: assign({
            workspaces: ({workspaces}, {data}) =>
              workspaces.filter(({slug}) => slug !== data.slug),
          }),
        },

        onError: {
          target: "error",
          actions: assign({error: (_ctx, {data}) => data.message}),
        },
      },
    },

    dashboard: {
      on: {
        SHOW_WORKSPACE: "show_workspace",
        RESTART_APP: "onboarding",
        RELOAD_WORKSPACES: "list_workspaces",
        CREATE_WORKSPACE: "create",
        LINK_WORKSPACE: "link",
        DELETE_WORKSPACE: "confirm_delete",
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
});
