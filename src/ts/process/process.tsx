import {useMachine} from "@xstate/react";
import React from "react";

import Error from "../common/error";
import Fatal from "../common/fatal";
import Modal from "../common/modal";
import {useAppCtx} from "../context";
import {listProcesses, runProcess, updateProcessConfig} from "../http";
import machine from "../machines/process";
import {Process as ProcessType, ProcessConfigReq, Workspace} from "../types";
import {useServiceLogger} from "../utils";
import ProcessConfig from "./process-config";
import ProcessList from "./process-list";

interface ProcessProps {
  workspace: Workspace;
}

const Process = ({workspace}: ProcessProps) => {
  const [state, send, service] = useMachine(machine, {
    services: {
      fetchProcesses: (_ctx, _ev) => listProcesses(workspace.slug),

      storeProcessConfig: (_ctx, {config}) =>
        updateProcessConfig(workspace.slug, config),

      runProcess: (_ctx, {process: {key}}) =>
        runProcess(workspace.slug, {key, kind: "all"}),
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
    case state.matches("configure"):
      return <div />;

    case state.matches("home"):
      return (
        <ProcessList
          onClick={(p: ProcessType) => send("SHOW_DETAILS", {process: p})}
          onRun={(p: ProcessType) => send("RUN", {process: p})}
          processes={processes}
        />
      );

    case state.matches("details"):
      switch (state.event.type) {
        case "SHOW_DETAILS": {
          return (
            <div>
              <Modal
                onCancel={() => send("SHOW_HOME")}
                title="Setup Process"
                description="Describing this modal"
              >
                <ProcessConfig
                  process={state.event.process}
                  onCancel={() => send("SHOW_HOME")}
                  onDone={(config: ProcessConfigReq) => {
                    send("SAVE_CONFIG", {config});
                  }}
                />
              </Modal>
              <ProcessList
                onClick={(p: ProcessType) => send("SHOW_DETAILS", {process: p})}
                onRun={(p: ProcessType) => send("RUN", {process: p})}
                processes={processes}
              />
            </div>
          );
        }

        default:
          return (
            <Fatal
              msg={`Sources table didn't match any valid state: ${state.value}`}
              reset={() => send("RETRY")}
            />
          );
      }

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
