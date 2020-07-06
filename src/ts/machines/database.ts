import {createMachine} from "xstate";

import {Workspace} from "../types";

type DatabaseContext = {
  workspace: Workspace;
  error?: string;
};

type DatabaseEvent =
  | {type: "SHOW_HOME"}
  | {type: "SHOW_DATA"}
  | {type: "RETRY"};

type DatabaseState =
  | {
      value: "home" | "exploration" | "list_data";
      context: DatabaseContext;
    }
  | {
      value: "error";
      context: DatabaseContext & {error: string};
    };

export default createMachine<DatabaseContext, DatabaseEvent, DatabaseState>({
  id: "database",
  initial: "home",
  states: {
    home: {
      on: {
        SHOW_DATA: "exploration",
      },
    },

    exploration: {
      on: {
        SHOW_HOME: "home",
      },
    },

    error: {
      on: {
        RETRY: "home",
      },
    },
  },
});
