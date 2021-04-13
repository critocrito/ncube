import {useMachine} from "@xstate/react";
import React from "react";

import Header from "../components/discovery-header";
import Introduction from "../components/discovery-introduction";
import Source from "../components/discovery-source";
import Workspaces from "../components/discovery-workspaces";
import Error from "../components/error";
import Placeholder from "../components/placeholder";
import Unreachable from "../components/unreachable";
import {useServiceLogger} from "../lib/hooks";
import machine from "../machines/discovery";

const Popup = () => {
  const [state, send, service] = useMachine(machine);

  useServiceLogger(service, machine.id);

  if (
    state.matches("initial") ||
    state.matches("healthy") ||
    state.matches("workspaces") ||
    state.matches("store_source")
  )
    return <Placeholder />;

  if (state.matches("introduction")) {
    const {sourceReq} = state.context;

    return (
      <Introduction sourceReq={sourceReq} onNext={() => send("PRESERVE")} />
    );
  }

  if (state.matches("workspaces_select")) {
    const {workspaces} = state.context;

    return (
      <Workspaces
        onNext={(workspace) => send("SELECT_WORKSPACE", {workspace})}
        workspaces={workspaces}
      />
    );
  }

  if (state.matches("source")) {
    const {sourceReq, workspace} = state.context;

    return (
      <Source
        onNext={(source) => send("STORE_SOURCE", {sourceReq: source})}
        onCancel={() => send("CLOSE")}
        workspace={workspace}
        sourceReq={sourceReq}
      />
    );
  }

  if (state.matches("success"))
    return (
      <div className="mt4 pa4 bg-washed-green">
        <span className="b success">Success</span>
      </div>
    );

  if (state.matches("error")) {
    const {error} = state.context;

    return (
      <Error
        msg={error || "Failed to communicate with Ncube. Is it running?"}
        recover={() => send("RETRY")}
      />
    );
  }

  return <Unreachable machine={machine.id} state={state.value} />;
};

export default () => (
  <div style={{width: "500px"}}>
    <div className="pa4">
      <Header />

      <Popup />
    </div>
  </div>
);
