import {useMachine} from "@xstate/react";
import React from "react";

import BasicPanel from "../common/basic-panel";
import Fatal from "../common/fatal";
import FormHandler from "../common/form-handler";
import OnboardingForm, {OnboardingFormValues} from "../forms/onboarding";
import {createConfig, registerClient, showConfig} from "../http";
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

  return createConfig(body);
};

const Onboarding = ({onDone}: OnboardingProps) => {
  const [state, send, service] = useMachine(machine, {
    actions: {
      done: (_ctx) => onDone(),
    },

    services: {
      fetchData: async (_ctx, _ev) => showConfig(),

      registerClient: async (_ctx, _ev) => {
        // FIXME: Refactor this into a more manageable subscription structure.
        const data = await registerClient();

        const ws = new WebSocket(data.url);

        // ws.addEventListener("open", function open() {
        //   console.log("connected");
        //   ws.send(Date.now().toString());
        // });
        //
        ws.addEventListener("message", function subscribe(event) {
          console.log("Message from server:", event.data);
        });

        return data;
      },
    },
  });

  useServiceLogger(service, machine.id);

  switch (true) {
    case state.matches("showConfig"):
    case state.matches("registerClient"):
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
