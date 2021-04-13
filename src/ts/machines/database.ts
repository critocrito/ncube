import {
  ActorRefFrom,
  assign,
  createMachine,
  DoneInvokeEvent,
  Interpreter,
  sendParent,
} from "xstate";

import {defaultPageSize} from "../lib/constants";
import {listSegments, listUnits, searchUnits, statDataTotal} from "../lib/http";
import {spawn} from "../lib/utils";
import {DataStats, SearchResults, Segment, Unit, Workspace} from "../types";
import tableMachine, {TableMachineInterpreter} from "./table";

export type DatabaseContext = {
  workspace: Workspace;
  units: Unit[];
  selectedUnits: Unit[];
  segments: Segment[];
  dataStats: DataStats;
  searchQuery: string;
  total: number;
  tableRef?: TableMachineInterpreter;
  unit?: Unit;
  segment?: Segment;
  error?: string;
};

export type DatabaseEventShow = {type: "SHOW"; unit: Unit};

export type DatabaseEventShowSegment = {type: "SHOW_SEGMENT"; segment: Segment};

export type DatabaseEventCreateSegment = {
  type: "CREATE_SEGMENT";
};

export type DatabaseEventUpdateSegment = {
  type: "UPDATE_SEGMENT";
};

export type DatabaseEventDeleteSegment = {
  type: "DELETE_SEGMENT";
  segment: Segment;
};

export type DatabaseEventUpdateQuery = {type: "SET_QUERY"; query: string};

export type DatabaseEventPaginate = {
  type: "PAGINATE";
  pageIndex: number;
  pageSize: number;
};

export type DatabaseEvent =
  | DatabaseEventShow
  | {type: "SHOW_HOME"}
  | {type: "SHOW_DATA"}
  | {type: "SHOW_HELP"}
  | DatabaseEventShowSegment
  | {type: "SEND_TO_VERIFY"; segment: Segment}
  | DatabaseEventCreateSegment
  | DatabaseEventUpdateSegment
  | DatabaseEventDeleteSegment
  | {type: "RELOAD"}
  | DatabaseEventUpdateQuery
  | DatabaseEventPaginate
  | {type: "RETRY"};

export type DatabaseState =
  | {value: "initialize"; context: DatabaseContext}
  | {
      value: "units" | "segments" | "home" | "segment_verify";
      context: DatabaseContext & {tableRef: TableMachineInterpreter};
    }
  | {
      value: "exploration" | "help" | "segment_create";
      context: DatabaseContext & {tableRef: TableMachineInterpreter};
    }
  | {
      value: "details";
      context: DatabaseContext & {
        tableRef: TableMachineInterpreter;
        unit: Unit;
      };
    }
  | {
      value: "segment_update" | "segment_delete";
      context: DatabaseContext & {
        tableRef: TableMachineInterpreter;
        segment: Segment;
      };
    }
  | {
      value: "error";
      context: DatabaseContext & {
        tableRef: TableMachineInterpreter;
        error: string;
      };
    };

export type DatabaseMachineInterpreter = ActorRefFrom<
  Interpreter<DatabaseContext, DatabaseState, DatabaseEvent>["machine"]
>;

export default createMachine<DatabaseContext, DatabaseEvent, DatabaseState>(
  {
    id: "database",
    initial: "initialize",
    states: {
      initialize: {
        entry: "spawnTable",

        always: "segments",
      },

      segments: {
        entry: "resetContext",

        invoke: {
          src: "listSegments",
          onDone: {
            target: "home",
            actions: "setSegments",
          },
          onError: {
            target: "error",
            actions: "fail",
          },
        },
      },

      units: {
        invoke: {
          src: "listUnits",
          onDone: {
            target: "exploration",
            actions: "setUnits",
          },
          onError: {
            target: "error",
            actions: "fail",
          },
        },
      },

      home: {
        entry: "resetContext",

        on: {
          SHOW_DATA: "units",
          SHOW_SEGMENT: {
            target: "units",
            actions: "setSegment",
          },
          SEND_TO_VERIFY: "segment_verify",
          DELETE_SEGMENT: "segment_delete",
        },
      },

      exploration: {
        entry: "setHeader",

        on: {
          SHOW_HOME: {
            target: "segments",
            actions: "unsetSegment",
          },
          SHOW_HELP: "help",
          SHOW_DATA: "units",
          SHOW: {
            target: "details",
            actions: "setUnit",
          },
          CREATE_SEGMENT: {
            target: "segment_create",
            cond: "hasNoSegment",
          },
          UPDATE_SEGMENT: {
            target: "segment_update",
            cond: "hasSegment",
          },
          SET_QUERY: {
            actions: "setSearchQuery",
          },
          PAGINATE: "units",
        },
      },

      help: {
        on: {
          SHOW_HOME: "exploration",
        },
      },

      details: {
        on: {
          SHOW_HOME: "exploration",
        },
      },

      segment_create: {
        on: {
          SHOW_DATA: "exploration",
        },
      },

      segment_update: {
        on: {
          SHOW_DATA: "exploration",
        },
      },

      segment_delete: {
        entry: "setSegment",
        exit: "resetContext",

        on: {
          RELOAD: "segments",
        },
      },

      segment_verify: {
        on: {
          SHOW_HOME: "segments",
        },
      },

      error: {
        on: {
          RETRY: "segments",
        },
      },
    },
  },

  {
    actions: {
      spawnTable: assign<DatabaseContext>({
        tableRef: ({dataStats}) =>
          spawn(
            tableMachine.withContext({
              pageIndex: 0,
              pageSize: defaultPageSize,
              total: dataStats.total,
              selected: [],
            }),
            "dataTable",
          ),
      }),

      setHeader: sendParent((_ctx, ev) => {
        const {segment} = ev as DatabaseEventShowSegment;

        if (segment)
          return {
            type: "HEADER",
            header: `Database: ${segment.title}`,
          };
        return {type: "HEADER", header: `Database`};
      }),

      setUnits: assign((_ctx, ev) => {
        const {
          data: {data, total},
        } = ev as DoneInvokeEvent<SearchResults<Unit>>;
        return {
          total,
          units: data,
        };
      }),

      setUnit: assign({
        unit: (_ctx, ev) => {
          const {unit} = ev as DatabaseEventShow;
          return unit;
        },
      }),

      setSegments: assign({
        segments: (_ctx, ev) => {
          const {data} = ev as DoneInvokeEvent<Segment[]>;

          return data;
        },
      }),

      setSegment: assign((_ctx, ev) => {
        const {segment} = ev as DatabaseEventDeleteSegment;

        return {
          segment,
          searchQuery: segment.query,
        };
      }),

      unsetSegment: assign((ctx) => ({
        ...ctx,
        segment: undefined,
      })),

      setSearchQuery: assign({
        searchQuery: (_ctx, ev) => {
          const {query} = ev as DatabaseEventUpdateQuery;
          return query;
        },
      }),

      resetContext: assign((ctx) => ({
        ...ctx,
        searchQuery: "",
        segment: undefined,
        error: undefined,
      })),

      fail: assign({
        error: (_ctx, ev) => {
          const {data} = ev as DoneInvokeEvent<Error>;
          return data.message;
        },
      }),
    },

    guards: {
      hasSegment: (ctx): boolean => ctx.segment !== undefined,

      hasNoSegment: (ctx): boolean => ctx.segment === undefined,
    },

    services: {
      listUnits: async (
        {workspace, searchQuery},
        ev,
      ): Promise<SearchResults<Unit>> => {
        const {pageIndex, pageSize} = {
          pageIndex: 0,
          pageSize: defaultPageSize,
          ...ev,
        };

        if (searchQuery === "") {
          const [units, total] = await Promise.all([
            listUnits(workspace.slug, pageIndex, pageSize),
            statDataTotal(workspace.slug),
          ]);

          return {data: units, total};
        }

        return searchUnits(workspace.slug, searchQuery, pageIndex, pageSize);
      },

      listSegments: ({workspace}): Promise<Segment[]> =>
        listSegments(workspace.slug),
    },
  },
);
