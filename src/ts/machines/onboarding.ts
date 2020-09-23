import {createMachine} from "xstate";

// eslint-disable-next-line @typescript-eslint/no-empty-interface
interface OnboardingContext {}

type OnboardingEvent = {type: "SHOW_CONFIG"};

type OnboardingState = {
  value: "showConfig" | "registerClient" | "bootstrap" | "done";
  context: OnboardingContext;
};

export default createMachine<
  OnboardingContext,
  OnboardingEvent,
  OnboardingState
>({
  id: "onboarding",
  context: {},
  initial: "showConfig",
  states: {
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

    done: {
      entry: "done",
      type: "final",
    },
  },
});
