import {assign, createMachine} from "xstate";

import {Workspace} from "../types";

interface WorkspaceContext {
  workspaces: Workspace[];
  current: Workspace | undefined;
}

type WorkspaceEvent = {type: "RETRY"};

type WorkspaceState =
  | {
      value: "prepareData";
      context: WorkspaceContext;
    }
  | {
      value: "overview";
      context: WorkspaceContext;
    }
  | {
      value: "workspaceError";
      context: WorkspaceContext;
    };

export default createMachine<WorkspaceContext, WorkspaceEvent, WorkspaceState>({
  id: "workspace",
  context: {workspaces: [], current: undefined},
  initial: "prepareData",
  states: {
    prepareData: {
      invoke: {
        src: "prepareData",
        onDone: {
          target: "overview",
          actions: assign({
            workspaces: (_, {data}) => data.workspaces,
            current: (_, {data}) => data.current,
          }),
        },
        onError: {
          target: "workspaceError",
        },
      },
    },
    workspaceError: {on: {RETRY: "prepareData"}},
    overview: {},
  },
});
