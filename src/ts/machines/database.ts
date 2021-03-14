import {assign, createMachine} from "xstate";

import {Segment, Workspace} from "../types";

type DatabaseContext = {
  workspace: Workspace;
  segments: Segment[];
  error?: string;
};

export type DatabaseEventReallyDelete = {
  type: "REALLY_DELETE_SEGMENT";
  segment: Segment;
};

type DatabaseEvent =
  | {type: "SHOW_HOME"}
  | {type: "SHOW_DATA"}
  | {type: "SHOW_SEGMENT"; segment: Segment}
  | {type: "SEND_TO_VERIFY"; segment: Segment}
  | {type: "DELETE_SEGMENT"; segment: Segment}
  | DatabaseEventReallyDelete
  | {type: "RETRY"};

type DatabaseState =
  | {
      value:
        | "segments"
        | "home"
        | "exploration"
        | "verify_segment"
        | "delete_segment"
        | "confirm_delete_segment";
      context: DatabaseContext;
    }
  | {
      value: "error";
      context: DatabaseContext & {error: string};
    };

export default createMachine<DatabaseContext, DatabaseEvent, DatabaseState>({
  id: "database",
  initial: "segments",
  states: {
    segments: {
      invoke: {
        src: "fetchSegments",

        onDone: {
          target: "home",
          actions: assign({segments: (_, {data}) => data}),
        },

        onError: {
          target: "error",
          actions: assign({error: (_ctx, {data}) => data.message}),
        },
      },
    },

    delete_segment: {
      invoke: {
        src: "deleteSegment",

        onDone: {
          target: "home",
          actions: assign({
            segments: ({segments}, {data}) =>
              segments.filter(({slug}) => slug !== data.slug),
          }),
        },

        onError: {
          target: "error",
          actions: assign({error: (_ctx, {data}) => data.message}),
        },
      },
    },

    home: {
      on: {
        SHOW_DATA: "exploration",
        SEND_TO_VERIFY: "verify_segment",
        SHOW_SEGMENT: "exploration",
        DELETE_SEGMENT: "confirm_delete_segment",
      },
    },

    exploration: {
      on: {
        SHOW_HOME: "home",
      },
    },

    verify_segment: {
      on: {
        SHOW_HOME: "home",
      },
    },

    confirm_delete_segment: {
      on: {
        SHOW_HOME: "home",
        REALLY_DELETE_SEGMENT: "delete_segment",
      },
    },

    error: {
      on: {
        RETRY: "home",
      },
    },
  },
});
