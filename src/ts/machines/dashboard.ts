import {
  ActorRefFrom,
  assign,
  createMachine,
  DoneInvokeEvent,
  Interpreter,
  sendParent,
} from "xstate";

import {listWorkspaces} from "../lib/handlers";
import {showWorkspace} from "../lib/http";
import {ConnectionDetails, Workspace} from "../types";

export interface DashboardContext {
  workspaces: Workspace[];
  workspace?: Workspace;
  connection?: ConnectionDetails;
  error?: string;
}

export type DashboardEventWorkspace = {
  type: "SHOW_WORKSPACE";
  workspace: Workspace;
};

export type DashboardEventDelete = {
  type: "DELETE_WORKSPACE";
  workspace: Workspace;
};

export type DashboardEventLinkConnection = {
  type: "LINK_WORKSPACE";
  details: ConnectionDetails;
};

export type DashboardEventError = {type: "ERROR"; error: string};

export type DashboardEvent =
  | DashboardEventWorkspace
  | {type: "CREATE_WORKSPACE"}
  | DashboardEventLinkConnection
  | {type: "LINK_CONNECTION"}
  | DashboardEventDelete
  | {type: "CANCEL"}
  | DashboardEventError
  | {type: "RELOAD"}
  | {type: "RETRY"};

export type DashboardState =
  | {
      value: "workspaces" | "dashboard" | "workspace" | "create" | "connection";
      context: DashboardContext;
    }
  | {
      value: "link";
      context: DashboardContext & {connection: ConnectionDetails};
    }
  | {
      value: "delete" | "details";
      context: DashboardContext & {workspace: Workspace};
    }
  | {
      value: "error";
      context: DashboardContext & {error: string};
    };

export type DashboardMachineInterpreter = ActorRefFrom<
  Interpreter<DashboardContext, DashboardState, DashboardEvent>["machine"]
>;

export default createMachine<DashboardContext, DashboardEvent, DashboardState>(
  {
    id: "dashboard",

    initial: "workspaces",

    context: {
      workspaces: [],
    },

    states: {
      dashboard: {
        on: {
          RELOAD: "dashboard",
          SHOW_WORKSPACE: {
            target: "workspace",
            actions: "selectWorkspace",
          },
          CREATE_WORKSPACE: "create",
          LINK_CONNECTION: "connection",
          DELETE_WORKSPACE: {
            target: "delete",
            actions: "deleteWorkspace",
          },
        },
      },

      create: {
        on: {
          CANCEL: "workspaces",
          RELOAD: "workspaces",
        },
      },

      connection: {
        on: {
          LINK_WORKSPACE: {
            target: "link",
            actions: "setConnectionDetails",
          },
          ERROR: {
            target: "error",
            actions: "failUpload",
          },
          RELOAD: "workspaces",
        },
      },

      link: {
        on: {
          RELOAD: "workspaces",
        },
      },

      details: {
        entry: "syncWorkspace",

        on: {
          RELOAD: "workspaces",
        },
      },

      delete: {
        exit: "resetContext",

        on: {
          RELOAD: "workspaces",
        },
      },

      workspaces: {
        invoke: {
          src: "listWorkspaces",
          onDone: {
            target: "dashboard",
            actions: ["setWorkspaces", "syncWorkspaces"],
          },
          onError: {
            target: "error",
            actions: "fail",
          },
        },
      },

      workspace: {
        invoke: {
          src: "showWorkspace",
          onDone: {
            target: "details",
            actions: "setWorkspace",
          },
          onError: {
            target: "error",
            actions: "fail",
          },
        },
      },

      error: {
        on: {
          RETRY: "workspaces",
        },
      },
    },
  },
  {
    actions: {
      setWorkspaces: assign({
        workspaces: (_ctx, ev) => {
          const {data} = ev as DoneInvokeEvent<Workspace[]>;
          return data;
        },
      }),

      setWorkspace: assign({
        workspace: (_ctx, ev) => {
          const {data} = ev as DoneInvokeEvent<Workspace>;
          return data;
        },
      }),

      deleteWorkspace: assign({
        workspace: (_ctx, ev) => {
          const {workspace} = ev as DashboardEventDelete;
          return workspace;
        },
      }),

      selectWorkspace: assign({
        workspace: (_ctx, ev) => {
          const {workspace} = ev as DashboardEventWorkspace;
          return workspace;
        },
      }),

      setConnectionDetails: assign({
        connection: (_ctx, ev) => {
          const {details} = ev as DashboardEventLinkConnection;
          return details;
        },
      }),

      syncWorkspace: sendParent(({workspace, workspaces}) => ({
        type: "SHOW_WORKSPACE",
        workspace,
        workspaces,
      })),

      syncWorkspaces: sendParent(({workspaces}) => ({
        type: "SYNC_WORKSPACES",
        workspaces,
      })),

      resetContext: assign((ctx) => ({
        ...ctx,
        workspace: undefined,
        connection: undefined,
        error: undefined,
      })),

      failUpload: assign({
        error: (_, ev) => {
          const {error} = ev as DashboardEventError;
          return error;
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
      showWorkspace: async (_ctx, ev): Promise<Workspace> => {
        const {
          workspace: {slug},
        } = ev as DashboardEventWorkspace;
        return showWorkspace(slug);
      },

      listWorkspaces,
    },
  },
);
