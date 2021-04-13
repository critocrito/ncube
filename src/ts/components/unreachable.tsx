import React from "react";
import {StateValue} from "xstate";

import {useNcubeCtx} from "../lib/context";
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
      msg={`${machine} machine didn't match any valid state: ${state}`}
      reset={reset || (() => send("RESTART_APP"))}
    />
  );
};

export default Unreachable;
