import {useMachine} from "@xstate/react";
import React from "react";

import Error from "../common/error";
import Fatal from "../common/fatal";
import {useAppCtx} from "../context";
import {listProcesses} from "../http";
import machine from "../machines/process";
import {Workspace} from "../types";
import {useServiceLogger} from "../utils";
import ProcessCard from "./process-card";

interface ProcessProps {
  workspace: Workspace;
}

const Process = ({workspace}: ProcessProps) => {
  const [state, send, service] = useMachine(machine, {
    services: {
      fetchProcesses: (_ctx, _ev) => listProcesses(workspace.slug),
    },

    context: {
      workspace,
      processes: [],
    },
  });

  useServiceLogger(service, machine.id);

  const [, appSend] = useAppCtx();

  const {error, processes} = state.context;

  switch (true) {
    case state.matches("processes"):
      return <div />;

    case state.matches("home"):
      return (
        <div className="flex flex-column">
          {processes.map((process) => (
            <ProcessCard key={process.id} process={process} />
          ))}
        </div>
      );

    case state.matches("error"):
      return (
        <Error
          msg={error || "Failed to fetch processes."}
          recover={() => send("RETRY")}
        />
      );

    default:
      return (
        <Fatal
          msg={`Process route didn't match any valid state: ${state.value}`}
          reset={() => appSend("RESTART_APP")}
        />
      );
  }
};

export default Process;
