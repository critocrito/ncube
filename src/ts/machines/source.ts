import {
  ActorRefFrom,
  assign,
  createMachine,
  DoneInvokeEvent,
  Interpreter,
} from "xstate";

import {defaultPageSize} from "../lib/constants";
import {listSources, statSourcesTotal} from "../lib/http";
import {spawn} from "../lib/utils";
import {SearchResults, Source, SourceStats, Workspace} from "../types";
import tableMachine, {TableMachineInterpreter} from "./table";

export type SourceContext = {
  workspace: Workspace;
  sources: Source[];
  selectedSources: Source[];
  sourceStats: SourceStats;
  total: number;
  tableRef?: TableMachineInterpreter;
  source?: Source;
  error?: string;
};

export type SourceEventShow = {type: "SHOW"; source: Source};

export type SourceEventDelete = {type: "DELETE"; source: Source};

export type SourceEventPaginate = {
  type: "PAGINATE";
  pageIndex: number;
  pageSize: number;
};

export type SourceEvent =
  | {type: "RELOAD"}
  | {type: "HOME"}
  | SourceEventShow
  | SourceEventDelete
  | SourceEventPaginate
  | {type: "CREATE"}
  | {type: "CANCEL"}
  | {type: "RETRY"};

export type SourceState =
  | {value: "initialize"; context: SourceContext}
  | {
      value: "sources" | "home" | "create";
      context: SourceContext & {tableRef: TableMachineInterpreter};
    }
  | {
      value: "details" | "delete";
      context: SourceContext & {
        tableRef: TableMachineInterpreter;
        source: Source;
      };
    }
  | {
      value: "error";
      context: SourceContext & {
        tableRef: TableMachineInterpreter;
        error: string;
      };
    };

export type SourceMachineInterpreter = ActorRefFrom<
  Interpreter<SourceContext, SourceState, SourceEvent>["machine"]
>;

export default createMachine<SourceContext, SourceEvent, SourceState>(
  {
    id: "source",

    initial: "initialize",

    states: {
      sources: {
        invoke: {
          src: "fetchSources",
          onDone: {
            target: "home",
            actions: "setSources",
          },
          onError: {
            target: "error",
            actions: "fail",
          },
        },
      },

      initialize: {
        entry: "spawnTable",

        always: "sources",
      },

      home: {
        entry: "resetContext",

        on: {
          SHOW: {
            target: "details",
            actions: "setSource",
          },
          CREATE: "create",
          DELETE: {
            target: "delete",
            actions: "setSource",
          },
          PAGINATE: "sources",
        },
      },

      create: {
        on: {
          CANCEL: "home",
          RELOAD: "sources",
        },
      },

      details: {
        on: {
          HOME: "home",
          DELETE: {
            target: "delete",
            actions: "setSource",
          },
        },
      },

      delete: {
        on: {
          HOME: "sources",
        },
      },

      error: {
        on: {
          RETRY: "sources",
        },
      },
    },
  },
  {
    actions: {
      spawnTable: assign<SourceContext>({
        tableRef: ({sourceStats}) =>
          spawn(
            tableMachine.withContext({
              pageIndex: 0,
              pageSize: 20,
              total: sourceStats.total,
              selected: [],
            }),
            "sourceTable",
          ),
      }),

      setSources: assign((_ctx, ev) => {
        const {
          data: {data, total},
        } = ev as DoneInvokeEvent<SearchResults<Source>>;

        return {total, sources: data};
      }),

      setSource: assign({
        source: (_ctx, ev) => {
          const {source} = ev as SourceEventShow;
          return source;
        },
      }),

      resetContext: assign((ctx) => ({
        ...ctx,
        source: undefined,
        error: undefined,
      })),

      fail: assign({
        error: (_ctx, ev) => {
          const {data} = ev as DoneInvokeEvent<Error>;
          return data.message;
        },
      }),
    },

    services: {
      fetchSources: async ({workspace}, ev): Promise<SearchResults<Source>> => {
        const {pageIndex, pageSize} = {
          pageIndex: 0,
          pageSize: defaultPageSize,
          ...ev,
        };

        const [sources, total] = await Promise.all([
          listSources(workspace.slug, pageIndex, pageSize),
          statSourcesTotal(workspace.slug),
        ]);
        return {data: sources, total};
      },
    },
  },
);
