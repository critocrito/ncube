import {assign, createMachine} from "xstate";

interface OnboardingContext {
  error?: string;
}

type OnboardingEvent = {type: "SHOW_CONFIG"} | {type: "RETRY"};

type OnboardingState =
  | {
      value:
        | "checkHealth"
        | "showConfig"
        | "registerClient"
        | "bootstrap"
        | "done";
      context: OnboardingContext;
    }
  | {
      value: "error";
      context: OnboardingContext & {error: string};
    };

export default createMachine<
  OnboardingContext,
  OnboardingEvent,
  OnboardingState
>({
  id: "onboarding",

  initial: "checkHealth",

  states: {
    checkHealth: {
      invoke: {
        src: "checkHealth",
        onDone: {
          target: "showConfig",
        },
        onError: {
          target: "error",
          actions: assign({error: (_ctx, {data}) => data.message}),
        },
      },
    },

    showConfig: {
      invoke: {
        src: "fetchData",
        onDone: {
          target: "registerClient",
        },
        onError: {
          target: "bootstrap",
        },
      },
    },

    registerClient: {
      invoke: {
        src: "registerClient",
        onDone: {
          target: "done",
        },
        onError: {
          target: "bootstrap",
        },
      },
    },

    bootstrap: {
      on: {
        SHOW_CONFIG: "showConfig",
      },
    },

    error: {
      on: {
        RETRY: "checkHealth",
      },
    },

    done: {
      entry: "done",
      type: "final",
    },
  },
});
