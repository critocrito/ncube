import {useMachine} from "@xstate/react";
import React from "react";

import Fatal from "../common/fatal";
import FormHandler from "../common/form-handler";
import OnboardingForm, {OnboardingFormValues} from "../forms/onboarding";
import {create, show} from "../http/config";
import BasicPanel from "../layout/basic-panel";
import machine from "../machines/onboarding";
import {ConfigSettingReq} from "../types";
import {useServiceLogger} from "../utils";

interface OnboardingProps {
  onDone: () => void;
}

const saveOnboardingForm = async (values: OnboardingFormValues) => {
  const body = Object.keys(values).map(
    (key: string): ConfigSettingReq => ({
      name: key,
      value: values[key as keyof typeof values],
    }),
  );

  return create(body);
};

const Onboarding = ({onDone}: OnboardingProps) => {
  const [state, send, service] = useMachine(machine, {
    actions: {
      done: (_ctx) => onDone(),
    },

    services: {
      fetchData: async (_ctx, _ev) => show(),
    },
  });

  useServiceLogger(service, machine.id);

  switch (true) {
    case state.matches("showConfig"):
      return <div />;

    case state.matches("bootstrap"):
      return (
        <BasicPanel
          header="Welcome to Ncube."
          description="Before you can start, please fill in some basic configuration."
        >
          <FormHandler
            onSave={saveOnboardingForm}
            onDone={() => send("SHOW_CONFIG")}
            Form={OnboardingForm}
          />
        </BasicPanel>
      );

    default:
      return (
        <Fatal
          msg={`${machine.id} machine didn't match any valid state: ${state.value}`}
        />
      );
  }
};

export default Onboarding;
