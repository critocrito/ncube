import {useActor} from "@xstate/react";
import React from "react";

import ProcessesIntroduction from "../../mdx/processes-intro.mdx";
import Error from "../components/error";
import IntroText from "../components/intro-text";
import Modal from "../components/modal";
import Panel from "../components/panel";
import Placeholder from "../components/placeholder";
import ProcessConfig from "../components/processes-config";
import ProcessesHome from "../components/processes-home";
import Unreachable from "../components/unreachable";
import {useWorkspaceCtx} from "../lib/context";
import machine, {ProcessMachineInterpreter} from "../machines/process";
import {Process, ProcessConfigReq} from "../types";

interface ProcessesProps {
  processRef: ProcessMachineInterpreter;
}

const Processes = ({processRef}: ProcessesProps) => {
  const [state, send] = useActor(processRef);

  const {processes} = state.context;

  if (
    state.matches("processes") ||
    state.matches("configure") ||
    state.matches("run")
  )
    return <Placeholder />;

  if (state.matches("home"))
    return (
      <ProcessesHome
        onShow={(process: Process) => send({type: "SHOW_DETAILS", process})}
        onRun={(process: Process) => send({type: "RUN", process})}
        processes={processes}
      />
    );

  if (state.matches("details") && state.event.type === "SHOW_DETAILS")
    return (
      <div>
        <Modal
          onCancel={() => send({type: "SHOW_HOME"})}
          title="Setup Process"
          description="Describing this modal"
        >
          <ProcessConfig
            process={state.event.process}
            onCancel={() => send({type: "SHOW_HOME"})}
            onDone={(config: ProcessConfigReq) => {
              send({type: "STORE_CONFIG", config});
            }}
          />
        </Modal>

        <ProcessesHome processes={processes} />
      </div>
    );

  if (state.matches("error")) {
    const {error} = state.context;

    return (
      <Error
        msg={error || "Failed to fetch processes."}
        recover={() => send({type: "RETRY"})}
      />
    );
  }

  return <Unreachable machine={machine.id} state={state.value} />;
};

export default () => {
  const [
    {
      context: {processRef, header},
    },
  ] = useWorkspaceCtx();

  return (
    <Panel header={header} description="">
      <>
        <IntroText>
          <ProcessesIntroduction />
        </IntroText>

        {processRef ? (
          <Processes processRef={processRef} />
        ) : (
          <Error msg="Processes actor is not available" recover={() => {}} />
        )}
      </>
    </Panel>
  );
};
