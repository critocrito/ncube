import {createMachine} from "xstate";

import {Workspace} from "../types";

type DatabaseContext = {
  workspace: Workspace;
};

type DatabaseEvent = {type: "SHOW_HOME"} | {type: "SHOW_DATA"};

type DatabaseState =
  | {
      value: "list_data";
      context: DatabaseContext;
    }
  | {
      value: "home" | "exploration";
      context: DatabaseContext;
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
  },
});
