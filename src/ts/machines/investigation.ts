import {assign, createMachine} from "xstate";

import {Investigation, Segment, Workspace} from "../types";

type InvestigationContext = {
  workspace: Workspace;
  investigations: Investigation[];
  error?: string;
};

type InvestigationEvent =
  | {type: "SHOW_DETAILS"; investigation: Investigation}
  | {type: "VERIFY_SEGMENT"; segment: Segment; investigation: Investigation}
  | {type: "CREATE_INVESTIGATION"}
  | {type: "SHOW_HOME"}
  | {type: "RETRY"};

type InvestigationState =
  | {
      value: "investigations" | "home" | "details" | "create" | "segment";
      context: InvestigationContext;
    }
  | {
      value: "error";
      context: InvestigationContext & {error: string};
    };

export default createMachine<
  InvestigationContext,
  InvestigationEvent,
  InvestigationState
>({
  id: "investigation",
  initial: "investigations",
  states: {
    investigations: {
      invoke: {
        src: "fetchInvestigations",

        onDone: {
          target: "home",
          actions: assign({investigations: (_, {data}) => data}),
        },

        onError: {
          target: "error",
          actions: assign({error: (_ctx, {data}) => data.message}),
        },
      },
    },

    home: {
      on: {
        SHOW_DETAILS: "details",
        CREATE_INVESTIGATION: "create",
      },
    },

    details: {
      on: {
        SHOW_HOME: "home",
        VERIFY_SEGMENT: "segment",
      },
    },

    create: {
      on: {
        SHOW_HOME: "investigations",
      },
    },

    segment: {
      on: {
        SHOW_DETAILS: "details",
        SHOW_HOME: "investigations",
      },
    },

    error: {
      on: {
        RETRY: "home",
      },
    },
  },
});
