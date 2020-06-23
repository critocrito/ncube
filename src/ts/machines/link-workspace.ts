import {assign, createMachine} from "xstate";

import {LinkWorkspaceFormValues} from "../forms/link-workspace";
import {ConnectionDetails} from "../types";

interface LinkWorkspaceContext {
  values?: Partial<LinkWorkspaceFormValues>;
  error?: string;
}

type LinkWorkspaceEvent =
  | {type: "UPLOAD_CONNECTION_DETAILS"}
  | {type: "NEXT"; details: ConnectionDetails}
  | {type: "ERROR"; error: string}
  | {type: "CANCEL"}
  | {type: "RETRY"}
  | {type: "DONE"};

type LinkWorkspaceState =
  | {
      value: "fileUpload";
      context: LinkWorkspaceContext;
    }
  | {
      value: "linkWorkspace";
      context: LinkWorkspaceContext & {
        values: ConnectionDetails;
      };
    }
  | {
      value: "error";
      context: LinkWorkspaceContext & {error: string; values: undefined};
    }
  | {value: "done"; context: LinkWorkspaceContext};

export default createMachine<
  LinkWorkspaceContext,
  LinkWorkspaceEvent,
  LinkWorkspaceState
>({
  id: "link-remote-workspace",
  context: {values: undefined, error: undefined},
  initial: "fileUpload",
  states: {
    fileUpload: {
      on: {
        CANCEL: "done",
        NEXT: {
          target: "linkWorkspace",
          actions: assign({
            values: (_ctx, ev) => ev.details,
          }),
        },
        ERROR: {
          target: "error",
          actions: assign({error: (_, {error}) => error}),
        },
      },
    },

    linkWorkspace: {
      on: {
        DONE: "done",
      },
    },

    error: {
      on: {
        RETRY: "fileUpload",
      },
    },

    done: {
      entry: "done",
      type: "final",
    },
  },
});
