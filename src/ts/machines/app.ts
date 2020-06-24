import {assign, createMachine} from "xstate";

import {Workspace} from "../types";

export interface AppContext {
  workspaces: Workspace[];
  workspace?: Workspace;
  error?: string;
}

export type AppEvent =
  | {type: "SHOW_HOME"}
  | {type: "SHOW_WORKSPACE"; slug: string}
  | {type: "RELOAD_WORKSPACES"}
  | {type: "RESTART_APP"}
  | {type: "RETRY"};

export type AppState =
  | {
      value: "onboarding" | "listing" | "fetchWorkspace" | "workspace" | "home";
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
    workspace: undefined,
  },

  initial: "onboarding",

  states: {
    onboarding: {
      on: {
        SHOW_HOME: "listing",
      },
    },

    listing: {
      invoke: {
        src: "listWorkspaces",

        onDone: {
          target: "home",
          actions: assign({workspaces: (_ctx, {data}) => data}),
        },

        onError: {
          target: "error",
          actions: assign({error: (_ctx, {data}) => data.message}),
        },
      },
    },

    fetchWorkspace: {
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

    home: {
      on: {
        SHOW_WORKSPACE: {
          target: "fetchWorkspace",
        },
        RELOAD_WORKSPACES: "listing",
        RESTART_APP: "onboarding",
      },
    },

    workspace: {
      on: {
        SHOW_WORKSPACE: {
          target: "fetchWorkspace",
        },
        RESTART_APP: "onboarding",
        SHOW_HOME: "home",
      },
    },

    error: {
      on: {
        RETRY: "listing",
      },
    },
  },
});
