import {createMachine} from "xstate";

import {HostConfig} from "../types";

// eslint-disable-next-line @typescript-eslint/no-empty-interface
interface OnboardingContext {}

type OnboardingEvent =
  | {type: "SAVE_CONFIG"; values: HostConfig}
  | {type: "RETRY"};

type OnboardingState =
  | {value: "showConfig"; context: OnboardingContext}
  | {value: "bootstrap"; context: OnboardingContext}
  | {value: "storeConfig"; context: OnboardingContext}
  | {value: "bootstrapError"; context: OnboardingContext}
  | {value: "bootstrapped"; context: OnboardingContext};

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
          target: "bootstrapped",
        },
        onError: {
          target: "bootstrap",
        },
      },
    },
    bootstrap: {
      on: {SAVE_CONFIG: "storeConfig"},
    },
    storeConfig: {
      invoke: {
        src: "storeData",
        onDone: {
          target: "showConfig",
        },
        onError: {
          target: "bootstrapError",
        },
      },
    },
    bootstrapError: {
      on: {RETRY: "bootstrap"},
    },
    bootstrapped: {
      entry: "finishBootstrap",
      type: "final",
    },
  },
});
