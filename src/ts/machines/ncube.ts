import {assign, createMachine, DoneInvokeEvent, send} from "xstate";

import {listWorkspaces} from "../lib/handlers";
import {showWorkspace} from "../lib/http";
import PubSub from "../lib/pubsub";
import {spawn} from "../lib/utils";
import {Workspace} from "../types";
import dashboardMachine, {DashboardMachineInterpreter} from "./dashboard";
import hostMachine, {HostMachineInterpreter} from "./host";
import workspaceMachine, {WorkspaceMachineInterpreter} from "./workspace";

export interface NcubeContext {
  workspaces: Workspace[];
  pubsub: PubSub;
  workspaceRefs: Record<string, WorkspaceMachineInterpreter>;
  hostRef?: HostMachineInterpreter;
  dashboardRef?: DashboardMachineInterpreter;
  workspaceRef?: WorkspaceMachineInterpreter;
  error?: string;
}

type NcubeEventWorkspace = {type: "SHOW_WORKSPACE"; workspace: Workspace};

type NcubeEventWorkspaces = {type: "SYNC_WORKSPACES"; workspaces: Workspace[]};

export type NcubeEvent =
  | {type: "CONNECTED"}
  | {type: "SHOW_DASHBOARD"}
  | {type: "RELOAD_DASHBOARD"}
  | NcubeEventWorkspace
  | NcubeEventWorkspaces
  | {type: "RESTART_APP"}
  | {type: "RETRY"};

export type NcubeState =
  | {
      value: "host";
      context: NcubeContext & {hostRef: HostMachineInterpreter};
    }
  | {
      value: "dashboard";
      context: NcubeContext & {
        hostRef: HostMachineInterpreter;
        dashboardRef: DashboardMachineInterpreter;
      };
    }
  | {
      value: "workspace";
      context: NcubeContext & {
        hostRef: HostMachineInterpreter;
        dashboardRef: DashboardMachineInterpreter;
        workspaceRef: WorkspaceMachineInterpreter;
        workspace: Workspace;
      };
    }
  | {
      value: "error";
      context: NcubeContext & {error: string};
    };

export default createMachine<NcubeContext, NcubeEvent, NcubeState>(
  {
    id: "ncube",

    initial: "host",

    context: {
      workspaces: [],
      pubsub: new PubSub(),
      workspaceRefs: {},
    },

    states: {
      host: {
        entry: "spawnHost",

        on: {
          CONNECTED: "dashboard",
        },
      },

      dashboard: {
        entry: "spawnDashboard",

        on: {
          SHOW_WORKSPACE: {
            target: "workspace",
            actions: "spawnWorkspace",
          },
          SYNC_WORKSPACES: {
            actions: "syncWorkspaces",
          },
          RELOAD_DASHBOARD: "dashboard",
          RESTART_APP: "host",
        },
      },

      workspace: {
        on: {
          SHOW_DASHBOARD: {
            target: "dashboard",
            actions: "reloadDashboard",
          },
          SHOW_WORKSPACE: {
            actions: "spawnWorkspace",
          },

          RESTART_APP: "host",
        },
      },

      error: {
        on: {
          RETRY: "host",
        },
      },
    },
  },
  {
    actions: {
      spawnHost: assign<NcubeContext>({
        hostRef: ({hostRef}) => hostRef || spawn(hostMachine, "host"),
      }),

      spawnDashboard: assign<NcubeContext>({
        dashboardRef: ({dashboardRef}) =>
          dashboardRef || spawn(dashboardMachine, "dashboard"),
      }),

      spawnWorkspace: assign<NcubeContext>((ctx, ev) => {
        const {workspace} = ev as NcubeEventWorkspace;
        let workspaceRef = ctx.workspaceRefs[workspace.slug];

        if (workspaceRef) return {...ctx, workspace, workspaceRef};

        workspaceRef = spawn(
          workspaceMachine.withContext({
            workspace,
            header: `Workspace: ${workspace.slug}`,
            dataStats: {
              total: 0,
              sources: 0,
              segments: 0,
              // videos: 0,
            },
            sourceStats: {
              total: 0,
              types: 0,
            },
          }),
          workspace.slug,
        );

        return {
          ...ctx,
          workspace,
          workspaceRef,
          workspaceRefs: {...ctx.workspaceRefs, [workspace.slug]: workspaceRef},
        };
      }),

      syncWorkspaces: assign<NcubeContext>({
        workspaces: (_ctx, ev) => {
          const {workspaces} = ev as NcubeEventWorkspaces;

          return workspaces;
        },
      }),

      reloadDashboard: send("RELOAD", {to: "dashboard"}),

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
        } = ev as NcubeEventWorkspace;
        return showWorkspace(slug);
      },

      listWorkspaces,
    },
  },
);
