import {useMachine} from "@xstate/react";
import React from "react";

import Error from "../components/error";
import FormHandler from "../components/form-handler";
import PanelBasic from "../components/panel-basic";
import Unreachable from "../components/unreachable";
import OnboardingForm, {OnboardingFormValues} from "../forms/onboarding";
import {
  createConfig,
  healthCheck,
  registerClient,
  showConfig,
} from "../lib/http";
import {useServiceLogger} from "../lib/utils";
import machine from "../machines/onboarding";
import {ConfigSettingReq} from "../types";

interface OnboardingProps {
  onDone: (url: string) => void;
}

const saveOnboardingForm = async (values: OnboardingFormValues) => {
  const body = Object.keys(values).map(
    (key: string): ConfigSettingReq => ({
      name: key,
      value: values[key as keyof typeof values],
    }),
  );

  return createConfig(body);
};

const Onboarding = ({onDone}: OnboardingProps) => {
  const [state, send, service] = useMachine(machine, {
    services: {
      checkHealth: async (_ctx, _ev) => healthCheck(),

      fetchData: async (_ctx, _ev) => showConfig(),

      registerClient: async (_ctx, _ev) => {
        const {url} = await registerClient();
        onDone(url);
      },
    },
  });

  useServiceLogger(service, machine.id);

  switch (true) {
    case state.matches("checkHealth") ||
      state.matches("showConfig") ||
      state.matches("registerClient"):
      return <div />;

    case state.matches("bootstrap"):
      return (
        <PanelBasic
          header="Welcome to Ncube."
          description="Before you can start, please fill in some basic configuration."
        >
          <FormHandler
            onSave={saveOnboardingForm}
            onDone={() => send("SHOW_CONFIG")}
            Form={OnboardingForm}
          />
        </PanelBasic>
      );

    case state.matches("error"):
      return (
        <Error
          msg={`Failed to connect to host: ${state.context.error}`}
          recover={() => send("RETRY")}
        />
      );

    default:
      return <Unreachable machine={machine.id} state={state.value} />;
  }
};

export default Onboarding;
