import React from "react";
import {StateValue} from "xstate";

import {useNcubeCtx} from "../lib/context";
import {capitalize} from "../lib/utils";
import Fatal from "./fatal";

interface UnreachableProps {
  machine: string;
  state: StateValue;
  reset?: () => void;
}

const Unreachable = ({machine, state, reset}: UnreachableProps) => {
  const [, send] = useNcubeCtx();

  return (
    <Fatal
      msg={`${capitalize(
        machine,
      )} machine didn't match any valid state: ${state}`}
      reset={reset || (() => send("RESTART_APP"))}
    />
  );
};

export default Unreachable;
