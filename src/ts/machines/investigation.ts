import {assign, createMachine, EventObject} from "xstate";

import {Investigation, Segment, SegmentUnit, Workspace} from "../types";

type InvestigationContext = {
  workspace: Workspace;
  investigations: Investigation[];
  segment?: Segment;
  investigation?: Investigation;
  unit?: SegmentUnit<Record<string, unknown>, EventObject>;
  error?: string;
};

type InvestigationEvent =
  | {type: "SHOW_DETAILS"; investigation: Investigation}
  | {
      type: "SHOW_UNIT";
      segment: Segment;
      investigation: Investigation;
      unitId: number;
      unit: SegmentUnit<Record<string, unknown>, EventObject>;
    }
  | {type: "VERIFY_SEGMENT"; segment: Segment; investigation: Investigation}
  | {type: "CREATE_INVESTIGATION"}
  | {type: "SHOW_HOME"}
  | {type: "RETRY"};

type InvestigationState =
  | {
      value: "investigations" | "home" | "create";
      context: InvestigationContext;
    }
  | {
      value: "details";
      context: InvestigationContext & {
        investigation: Investigation;
      };
    }
  | {
      value: "segment";
      context: InvestigationContext & {
        investigation: Investigation;
        segment: Segment;
      };
    }
  | {
      value: "unit";
      context: InvestigationContext & {
        investigation: Investigation;
        segment: Segment;
        unit: SegmentUnit<Record<string, unknown>, EventObject>;
      };
    }
  | {
      value: "error";
      context: InvestigationContext & {error: string};
    };

export default createMachine<
  InvestigationContext,
  InvestigationEvent,
  InvestigationState
>(
  {
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
        entry: ["resetContext"],
        on: {
          SHOW_DETAILS: {
            target: "details",
            actions: assign({
              investigation: (_ctx, {investigation}) => investigation,
            }),
          },
          CREATE_INVESTIGATION: "create",
        },
      },

      details: {
        on: {
          SHOW_HOME: "home",
          VERIFY_SEGMENT: {
            target: "segment",
            actions: assign((ctx, {segment, investigation}) => ({
              ...ctx,
              segment,
              investigation,
            })),
          },
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
          SHOW_UNIT: {
            target: "unit",
            actions: assign((ctx, {investigation, segment, unit}) => ({
              ...ctx,
              segment,
              investigation,
              unit,
            })),
          },
        },
      },

      unit: {
        on: {
          VERIFY_SEGMENT: {
            target: "segment",
            actions: assign((ctx, {segment, investigation}) => ({
              ...ctx,
              segment,
              investigation,
            })),
          },
        },
      },

      error: {
        on: {
          RETRY: "home",
        },
      },
    },
  },
  {
    actions: {
      resetContext: assign((ctx) => ({
        ...ctx,
        investigation: undefined,
        segment: undefined,
        unit: undefined,
        error: undefined,
      })),
    },
  },
);
