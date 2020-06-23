import {createMachine} from "xstate";

// eslint-disable-next-line @typescript-eslint/no-empty-interface
interface HomeContext {}

type HomeEvent =
  | {type: "CREATE_WORKSPACE"}
  | {type: "LINK_WORKSPACE"}
  | {type: "RETRY"};

type HomeState = {
  value: "home" | "create" | "link" | "done";
  context: HomeContext;
};

export default createMachine<HomeContext, HomeEvent, HomeState>({
  id: "home",
  context: {workspaces: []},
  initial: "home",
  states: {
    home: {
      on: {
        CREATE_WORKSPACE: "create",
        LINK_WORKSPACE: "link",
      },
    },

    create: {},

    link: {},

    done: {
      entry: "done",
      type: "final",
    },
  },
});
