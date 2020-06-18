import {navigate, RouteComponentProps} from "@reach/router";
import {useMachine} from "@xstate/react";
import React from "react";

import OnboardingForm from "../components/forms/onboarding";
import {create, show} from "../http/config";
import OnboardingMachine from "../machines/onboarding";
import {ConfigSettingReq} from "../types";
import {unreachable} from "../utils";

const Onboarding = (_: RouteComponentProps) => {
  const [state, send] = useMachine(OnboardingMachine, {
    actions: {
      finishBootstrap: (_ctx) => navigate("/home"),
    },

    services: {
      fetchData: async (_ctx, _ev) => show(),
      storeData: async (_ctx, {values}) => {
        const body = Object.keys(values).map(
          (key: string): ConfigSettingReq => ({
            name: key,
            value: values[key],
          }),
        );

        return create(body);
      },
    },
  });

  switch (state.value) {
    case "bootstrap":
      return (
        <div className="mw8 center">
          <div className="fl w-100 pa2">
            <h1 className="header1">Welcome to Ncube.</h1>
            <p>
              Before you can start, please fill in some basic configuration.
            </p>
            <OnboardingForm
              onSubmit={(values) => {
                send("SAVE_CONFIG", {values});
              }}
            />
          </div>
        </div>
      );
    case "showConfig":
    case "storeConfig":
      return <p>Loading</p>;
    case "bootstrapError":
      setTimeout(() => send("RETRY"), 5 * 1000);
      // FIXME: Handle this state better
      return <p className="error b">Bootstrap Error</p>;
    default:
      return unreachable("Onboarding route didn't match any valid state.");
  }
};

export default Onboarding;
