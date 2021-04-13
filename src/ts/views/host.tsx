import {useActor} from "@xstate/react";
import React from "react";

import Error from "../components/error";
import FormHandler from "../components/form-handler";
import PanelBasic from "../components/panel-basic";
import Placeholder from "../components/placeholder";
import Unreachable from "../components/unreachable";
import BootstrapForm, {BootstrapFormValues} from "../forms/bootstrap";
import {createConfig} from "../lib/http";
import machine, {HostMachineInterpreter} from "../machines/host";
import {ConfigSettingReq} from "../types";

interface HostProps {
  hostRef: HostMachineInterpreter;
}

const saveOnboardingForm = async (values: BootstrapFormValues) => {
  const body = Object.keys(values).map(
    (key: string): ConfigSettingReq => ({
      name: key,
      value: values[key as keyof typeof values],
    }),
  );

  return createConfig(body);
};

const Host = ({hostRef}: HostProps) => {
  const [state, send] = useActor(hostRef);

  if (state.matches("bootstrap"))
    return (
      <PanelBasic
        header="Welcome to Ncube."
        description="Before you can start, please fill in some basic configuration."
      >
        <FormHandler
          onSave={saveOnboardingForm}
          onDone={() => send({type: "INITIALIZE"})}
          Form={BootstrapForm}
        />
      </PanelBasic>
    );

  if (state.matches("error"))
    return (
      <Error
        msg={`Failed to connect to host: ${state.context.error}`}
        recover={() => send({type: "INITIALIZE"})}
      />
    );

  if (
    state.matches("initial") ||
    state.matches("healthy") ||
    state.matches("configuration") ||
    state.matches("registration") ||
    state.matches("connected")
  )
    return <Placeholder />;

  return <Unreachable machine={machine.id} state={state.value} />;
};

export default Host;
