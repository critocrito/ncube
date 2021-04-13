import {
  ActorRefFrom,
  assign,
  createMachine,
  DoneInvokeEvent,
  Interpreter,
  sendParent,
} from "xstate";

import {SearchResults, Source, Unit} from "../types";

type TableItem = {id: number};

export type TableContext<T extends TableItem> = {
  pageIndex: number;
  pageSize: number;
  total: number;
  selected: T[];
  error?: string;
};

export type TableEventSelection<T extends TableItem> = {
  type: "SET_SELECTION";
  selected: T[];
};

export type TableEventPagination = {
  type: "SET_PAGINATION";
  pageIndex: number;
  pageSize: number;
};

export type TableEvent<T extends TableItem> =
  | TableEventSelection<T>
  | TableEventPagination
  | {type: "RETRY"};

export type TableState<T extends TableItem> =
  | {
      value: "table";
      context: TableContext<T>;
    }
  | {
      value: "error";
      context: TableContext<T> & {error: string};
    };

export type TableMachineInterpreter = ActorRefFrom<
  Interpreter<
    TableContext<Unit | Source>,
    TableState<Unit | Source>,
    TableEvent<Unit | Source>
  >["machine"]
>;

export default createMachine<
  TableContext<Unit | Source>,
  TableEvent<Unit | Source>,
  TableState<Unit | Source>
>(
  {
    id: "table",

    initial: "table",

    states: {
      table: {
        on: {
          SET_SELECTION: {
            actions: [
              "setSelection",
              sendParent(({selected}) => ({type: "SET_SELECTION", selected})),
            ],
          },

          SET_PAGINATION: {
            actions: [
              "setPagination",
              sendParent(({pageIndex, pageSize}) => ({
                type: "PAGINATE",
                pageIndex,
                pageSize,
              })),
            ],
          },
        },
      },

      error: {
        on: {
          RETRY: "table",
        },
      },
    },
  },
  {
    actions: {
      setSelection: assign({
        selected: (_ctx, ev) => {
          const {selected} = ev as TableEventSelection<Unit | Source>;

          return selected;
        },
      }),

      setPagination: assign((_ctx, ev) => {
        const {pageIndex, pageSize} = ev as TableEventPagination;
        return {pageIndex, pageSize};
      }),

      setResults: assign((_ctx, ev) => {
        const {data} = ev as DoneInvokeEvent<SearchResults<Unit | Source>>;

        return {
          results: data.data,
          total: data.total,
          selected: [],
        };
      }),

      fail: assign({
        error: (_ctx, ev) => {
          const {data} = ev as DoneInvokeEvent<Error>;
          return data.message;
        },
      }),
    },
  },
);
