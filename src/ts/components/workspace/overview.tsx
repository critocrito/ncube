import React from "react";

import {useAppCtx, useWorkspaceCtx} from "../../lib/context";
import Panel from "../panel";
import SectionCard from "./section-card";

const WorkspaceOverview = () => {
  const [
    {
      context: {workspaces},
    },
  ] = useAppCtx();
  const [
    {
      context: {workspace, sourceStats, dataStats},
    },
    send,
  ] = useWorkspaceCtx();

  return (
    <Panel
      workspaces={workspaces}
      workspace={workspace}
      header={workspace.name}
      description={workspace.description}
    >
      <>
        <SectionCard
          onClick={() => send("SOURCE")}
          kind="source"
          stats={sourceStats}
        />
        <SectionCard
          onClick={() => send("DATA")}
          kind="data"
          stats={dataStats}
        />
        <SectionCard onClick={() => send("PROCESS")} kind="process" />
        <SectionCard
          onClick={() => send("INVESTIGATION")}
          kind="investigation"
        />
      </>
    </Panel>
  );
};

export default WorkspaceOverview;
