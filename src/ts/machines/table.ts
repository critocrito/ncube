import {assign, createMachine} from "xstate";

import {Source, Unit} from "../types";

export interface TableContext<T extends {id: number}> {
  query: string;
  pageIndex: number;
  pageSize: number;
  total: number;
  results: T[];
  selected: T[];
  error?: string;
}

export type TableEvent<T extends {id: number}> =
  | {type: "SHOW_TABLE"}
  | {type: "SHOW_DETAILS"; item: T}
  | {type: "SET_SELECTION"; selected: T[]}
  | {type: "SET_QUERY"; query: string}
  | {type: "SEARCH"; query: string; pageIndex: number; pageSize: number}
  | {type: "CREATE"}
  | {type: "DELETE"; item: T}
  | {type: "CONFIRM_DELETE"; id: number}
  | {type: "CANCEL"}
  | {type: "RETRY"};

export type TableState<T extends {id: number}> =
  | {
      value:
        | "fetching"
        | "deleting"
        | "table"
        | "details"
        | "create"
        | "delete";
      context: TableContext<T>;
    }
  | {
      value: "error";
      context: TableContext<T> & {error: string};
    };

export default createMachine<
  TableContext<Unit | Source>,
  TableEvent<Unit | Source>,
  TableState<Unit | Source>
>({
  id: "table",

  initial: "table",

  states: {
    fetching: {
      invoke: {
        src: "listItems",

        onDone: {
          target: "table",
          actions: assign((_ctx, {data}) => ({
            results: data.data,
            total: data.total,
            selected: [],
          })),
        },

        onError: {
          target: "error",
          actions: assign({error: (_ctx, {data}) => data.message}),
        },
      },
    },

    deleting: {
      invoke: {
        src: "deleteItem",

        onDone: {
          target: "fetching",
        },

        onError: {
          target: "error",
          actions: assign({error: (_ctx, {data}) => data.message}),
        },
      },
    },

    table: {
      on: {
        SHOW_DETAILS: "details",

        CREATE: "create",

        SEARCH: {
          target: "fetching",
          actions: assign((_ctx, {query, pageIndex, pageSize}) => ({
            query,
            pageIndex,
            pageSize,
          })),
        },

        SET_SELECTION: {
          target: "table",
          internal: true,
          actions: assign({selected: (_ctx, {selected}) => selected}),
        },

        SET_QUERY: {
          target: "table",
          internal: true,
          actions: assign({query: (_ctx, {query}) => query}),
        },

        DELETE: "delete",
      },
    },

    create: {
      on: {
        SHOW_TABLE: "fetching",
      },
    },

    delete: {
      on: {
        CANCEL: "table",
        CONFIRM_DELETE: "deleting",
      },
    },

    details: {
      on: {
        SHOW_TABLE: "table",
        DELETE: "delete",
      },
    },

    error: {
      on: {
        RETRY: "table",
      },
    },
  },
});
