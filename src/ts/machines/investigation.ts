import {
  ActorRefFrom,
  assign,
  createMachine,
  DoneInvokeEvent,
  Interpreter,
  Machine,
  sendParent,
} from "xstate";

import {
  listAnnotations,
  listInvestigations,
  listInvestigationSegments,
  listSegmentUnitsByState,
  setAnnotation,
  showMethodology,
  showUnit,
  updateUnitState,
} from "../lib/http";
import {unreachable} from "../lib/utils";
import {
  Annotation,
  Investigation,
  Segment,
  SegmentUnit,
  Unit,
  Verification,
  Workspace,
} from "../types";

type UnitDetails = {unit: Unit; annotations: Annotation[]};

export type InvestigationContext = {
  workspace: Workspace;
  investigations: Investigation[];
  segments: Segment[];
  verification?: Verification;
  segment?: Segment;
  investigation?: Investigation;
  unit?: SegmentUnit;
  unitDetails?: UnitDetails;
  error?: string;
};

export type InvestigationEventDetails = {
  type: "SHOW_DETAILS";
  investigation: Investigation;
};

export type InvestigationEventSegment = {
  type: "VERIFY_SEGMENT";
  segment: Segment;
};

export type InvestigationEventMoveUnit = {
  type: "MOVE_UNIT";
  unitId: number;
  from: string;
  to: string;
  position: number;
};

export type InvestigationEventUnit = {
  type: "SHOW_UNIT";
  unit: SegmentUnit;
};

export type InvestigationEventUpdateAnnotation = {
  type: "UPDATE_ANNOTATION";
  annotation: Annotation;
};

export type InvestigationEvent =
  | InvestigationEventDetails
  | InvestigationEventSegment
  | InvestigationEventUnit
  | InvestigationEventMoveUnit
  | InvestigationEventUpdateAnnotation
  | {type: "CREATE_INVESTIGATION"}
  | {type: "SHOW_HOME"}
  | {type: "RETRY"};

export type InvestigationState =
  | {
      value: "investigations" | "home" | "create";
      context: InvestigationContext;
    }
  | {
      value: "details" | "segments";
      context: InvestigationContext & {
        investigation: Investigation;
      };
    }
  | {
      value: "verification";
      context: InvestigationContext & {
        investigation: Investigation;
        segment: Segment;
      };
    }
  | {
      value: "segment_details" | "unit" | "progress";
      context: InvestigationContext & {
        investigation: Investigation;
        segment: Segment;
        verification: Verification;
      };
    }
  | {
      value: "unit_details" | "annotation";
      context: InvestigationContext & {
        investigation: Investigation;
        segment: Segment;
        verification: Verification;
        unit: SegmentUnit;
        unitDetails: UnitDetails;
      };
    }
  | {
      value: "error";
      context: InvestigationContext & {error: string};
    };

export type InvestigationMachineInterpreter = ActorRefFrom<
  Interpreter<
    InvestigationContext,
    InvestigationState,
    InvestigationEvent
  >["machine"]
>;

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
          src: "listInvestigations",
          onDone: {
            target: "home",
            actions: "setInvestigations",
          },
          onError: {
            target: "error",
            actions: "fail",
          },
        },
      },

      verification: {
        invoke: {
          src: "showVerification",
          onDone: {
            target: "segment_details",
            actions: "setVerification",
          },

          onError: {
            target: "error",
            actions: "fail",
          },
        },
      },

      annotations: {
        invoke: {
          src: "listAnnotations",
          onDone: {
            target: "unit_details",
            actions: "setAnnotations",
          },

          onError: {
            target: "error",
            actions: "fail",
          },
        },
      },

      annotation: {
        invoke: {
          src: "storeAnnotation",
          onDone: "annotations",

          onError: {
            target: "error",
            actions: "fail",
          },
        },
      },

      unit: {
        invoke: {
          src: "showUnit",
          onDone: {
            target: "unit_details",
            actions: "setUnitDetails",
          },

          onError: {
            target: "error",
            actions: "fail",
          },
        },
      },

      segments: {
        invoke: {
          src: "listSegments",
          onDone: {
            target: "details",
            actions: "setSegments",
          },

          onError: {
            target: "error",
            actions: "fail",
          },
        },
      },

      progress: {
        invoke: {
          src: "progressUnit",
          onDone: {
            target: "segment_details",
            actions: "setVerification",
          },

          onError: {
            target: "error",
            actions: "fail",
          },
        },
      },

      home: {
        on: {
          SHOW_DETAILS: {
            target: "segments",
            actions: ["setInvestigation", "setInvestigationHeader"],
          },
          CREATE_INVESTIGATION: "create",
        },
      },

      details: {
        on: {
          SHOW_HOME: "home",
          VERIFY_SEGMENT: {
            target: "verification",
            actions: ["setSegment", "setSegmentHeader"],
          },
        },
      },

      create: {
        on: {
          SHOW_HOME: "investigations",
        },
      },

      segment_details: {
        // enter: "unsetUnitDetails",

        on: {
          SHOW_HOME: "home",
          SHOW_DETAILS: "details",
          SHOW_UNIT: {
            target: "unit",
            actions: "setUnit",
          },
          MOVE_UNIT: "progress",
        },
      },

      unit_details: {
        on: {
          SHOW_HOME: "segment_details",
          UPDATE_ANNOTATION: "annotation",
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
      setInvestigations: assign({
        investigations: (_ctx, ev) => {
          const {data} = ev as DoneInvokeEvent<Investigation[]>;
          return data;
        },
      }),

      setInvestigation: assign<InvestigationContext>({
        investigation: (_ctx, ev) => {
          const {investigation} = ev as InvestigationEventDetails;
          return investigation;
        },
      }),

      setVerification: assign<InvestigationContext>({
        verification: (_ctx, ev) => {
          const {data} = ev as DoneInvokeEvent<Verification>;
          return data;
        },
      }),

      setSegments: assign({
        segments: (_ctx, ev) => {
          const {data} = ev as DoneInvokeEvent<Segment[]>;
          return data;
        },
      }),

      setSegment: assign({
        segment: (_ctx, ev) => {
          const {segment} = ev as InvestigationEventSegment;
          return segment;
        },
      }),

      setUnitDetails: assign({
        unitDetails: (_ctx, ev) => {
          const {data} = ev as DoneInvokeEvent<UnitDetails>;
          return data;
        },
      }),

      unsetUnitDetails: assign((ctx) => ({...ctx, unitDetails: undefined})),

      setAnnotations: assign({
        unitDetails: (ctx, ev) => {
          const {unitDetails} = ctx as InvestigationContext & {
            unitDetails: UnitDetails;
          };
          const {data} = ev as DoneInvokeEvent<Annotation[]>;
          return {...unitDetails, annotations: data};
        },
      }),

      setUnit: assign({
        unit: (_ctx, ev) => {
          const {unit} = ev as InvestigationEventUnit;
          return unit;
        },
      }),

      setInvestigationHeader: sendParent(({investigation}) => {
        if (!investigation)
          return unreachable("Failed to set investigation header");

        return {
          type: "HEADER",
          header: `Investigations: ${investigation.title}`,
        };
      }),

      setSegmentHeader: sendParent(({segment}) => {
        if (!segment)
          return unreachable("Failed to set investigation segment header");

        return {
          type: "HEADER",
          header: `Investigations: ${segment.title}`,
        };
      }),

      setHeader: sendParent(() => {
        return {type: "HEADER", header: "Investigations"};
      }),

      fail: assign({
        error: (_ctx, ev) => {
          const {data} = ev as DoneInvokeEvent<Error>;
          return data.message;
        },
      }),

      resetContext: assign((ctx) => ({
        ...ctx,
        investigations: [],
        investigation: undefined,
        segments: [],
        segment: undefined,
        verification: undefined,
        unit: undefined,
        unitDetails: undefined,
        error: undefined,
      })),
    },

    services: {
      listInvestigations: ({workspace}): Promise<Investigation[]> =>
        listInvestigations(workspace.slug),

      listSegments: (ctx): Promise<Segment[]> => {
        const {workspace, investigation} = ctx as InvestigationContext & {
          investigation: Investigation;
        };
        return listInvestigationSegments(workspace.slug, investigation.slug);
      },

      listAnnotations: (ctx): Promise<Annotation[]> => {
        const {workspace, investigation, unit} = ctx as InvestigationContext & {
          investigation: Investigation;
          unit: SegmentUnit;
        };

        return listAnnotations(
          workspace.slug,
          investigation.slug,
          unit.verification,
        );
      },

      showUnit: async (ctx): Promise<UnitDetails> => {
        const {
          workspace,
          investigation,
          unit: {id},
        } = ctx as InvestigationContext & {
          investigation: Investigation;
          unit: SegmentUnit;
        };

        const [unit, annotations] = await Promise.all([
          showUnit(workspace.slug, id),
          listAnnotations(workspace.slug, investigation.slug, id),
        ]);

        return {unit, annotations};
      },

      showVerification: async (ctx): Promise<Verification> => {
        const {
          workspace,
          investigation,
          segment,
        } = ctx as InvestigationContext & {
          investigation: Investigation;
          segment: Segment;
        };
        const methodology = await showMethodology(
          workspace.slug,
          investigation.methodology,
        );

        // We ensure that columns are in the right order.
        const states = Object.keys(methodology.process.states || {}).filter(
          (key) =>
            key !== "incoming_data" &&
            key !== "discarded_data" &&
            key !== "verified_data",
        );
        const columns = [
          "incoming_data",
          ...states,
          "discarded_data",
          "verified_data",
        ];

        const fetchedUnits = await Promise.all(
          columns.map((name) =>
            listSegmentUnitsByState(
              workspace.slug,
              investigation.slug,
              segment.slug,
              name,
            ).catch(() => []),
          ),
        );

        const units: Map<string, SegmentUnit[]> = new Map();
        [...columns.entries()].forEach(([i, name]) => {
          units.set(name, fetchedUnits[i] as SegmentUnit[]);
        });

        return {methodology, columns, units};
      },

      progressUnit: async (ctx, ev): Promise<Verification> => {
        const {
          workspace,
          investigation,
          segment,
          verification,
        } = ctx as InvestigationContext & {
          investigation: Investigation;
          segment: Segment;
          verification: Verification;
        };
        const {unitId, from, to, position} = ev as InvestigationEventMoveUnit;
        const {methodology, units} = verification;

        const sourceUnits = units.get(from);
        const destinationUnits = units.get(to);

        if (!sourceUnits || !destinationUnits)
          throw new Error(`Failed to find units for state ${from} or ${to}`);

        const unit = units.get(from)?.find(({id}) => id === unitId);

        if (!unit) throw new Error(`No unit found for id ${unitId}`);

        // Transition this piece of data to the next state.
        const machine = Machine(methodology.process);
        const currentState = machine.resolveState(unit.state);
        const event = `to_${to}`.toUpperCase();
        const nextState = machine.transition(currentState, event);

        if (!nextState.changed) return verification;

        // We have a valid transition and progress.
        await updateUnitState(
          workspace.slug,
          investigation.slug,
          segment.slug,
          unit.id,
          nextState,
        );

        unit.state = nextState;
        units.set(
          from,
          sourceUnits.filter(({id}) => id !== unit.id),
        );
        units.set(
          to,
          // Insert unit at the specified index.
          [
            ...destinationUnits.slice(0, position),
            unit,
            ...destinationUnits.slice(position),
          ],
        );

        return {...verification, units};
      },

      storeAnnotation: async (ctx, ev): Promise<void> => {
        const {workspace, investigation, unit} = ctx as InvestigationContext & {
          investigation: Investigation;
          unit: SegmentUnit;
        };
        const {annotation} = ev as InvestigationEventUpdateAnnotation;
        await setAnnotation(
          workspace.slug,
          investigation.slug,
          unit.verification,
          annotation,
        );
      },
    },
  },
);
