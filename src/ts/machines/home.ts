import {assign, createMachine} from "xstate";

import {ConnectionDetails, Workspace, WorkspaceReq} from "../types";

interface HomeContext {
  workspaces: Workspace[];
}

type HomeEvent =
  | {type: "CREATE_WORKSPACE"}
  | {type: "LINK_WORKSPACE"}
  | {type: "UPLOAD_CONNECTION_DETAILS"}
  | {type: "NEXT"; details: ConnectionDetails}
  | {type: "ERROR"; msg: string}
  | {type: "SAVE"; data: WorkspaceReq}
  | {type: "CANCEL"}
  | {type: "RETRY"};

type HomeState =
  | {value: "listWorkspaces"; context: HomeContext}
  | {value: "home"; context: HomeContext}
  | {value: "createWorkspace"; context: HomeContext}
  | {value: "uploadConnectionDetails"; context: HomeContext}
  | {value: "linkWorkspace"; context: HomeContext}
  | {value: "saveWorkspace"; context: HomeContext}
  | {value: "homeError"; context: HomeContext}
  | {value: "saveError"; context: HomeContext};

export default createMachine<HomeContext, HomeEvent, HomeState>({
  id: "home",
  context: {workspaces: []},
  initial: "listWorkspaces",
  states: {
    listWorkspaces: {
      invoke: {
        src: "listWorkspaces",
        onDone: {
          target: "home",
          actions: assign({workspaces: (_, {data}) => data}),
        },
        onError: {
          target: "homeError",
        },
      },
    },
    homeError: {
      on: {RETRY: "listWorkspaces"},
    },
    saveError: {
      on: {CANCEL: "home", RETRY: "saveWorkspace"},
    },
    home: {
      on: {
        CREATE_WORKSPACE: "createWorkspace",
        UPLOAD_CONNECTION_DETAILS: "uploadConnectionDetails",
      },
    },
    createWorkspace: {
      on: {
        CANCEL: "home",
        SAVE: "saveWorkspace",
      },
    },
    uploadConnectionDetails: {
      on: {
        CANCEL: "home",
        NEXT: "linkWorkspace",
        ERROR: "homeError",
      },
    },
    linkWorkspace: {
      on: {
        CANCEL: "home",
        SAVE: "saveWorkspace",
      },
    },
    saveWorkspace: {
      invoke: {
        src: "saveWorkspace",
        onDone: {
          target: "listWorkspaces",
        },
        onError: {
          target: "saveError",
        },
      },
    },
  },
});
