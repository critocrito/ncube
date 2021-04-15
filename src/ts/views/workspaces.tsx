import React from "react";

import Panel from "../components/panel";
import WorkspacesCard from "../components/workspaces-card";
import {useWorkspaceCtx} from "../lib/context";

const WorkspacesHome = () => {
  const [
    {
      context: {workspace, sourceStats, dataStats, header},
    },
    send,
  ] = useWorkspaceCtx();

  return (
    <Panel header={header} description={workspace.description}>
      <div className="space-y-8">
        <WorkspacesCard
          onShow={() => send({type: "SOURCE"})}
          kind="source"
          stats={sourceStats}
        />
        <WorkspacesCard
          onShow={() => send({type: "DATA"})}
          kind="data"
          stats={dataStats}
        />
        <WorkspacesCard onShow={() => send({type: "PROCESS"})} kind="process" />
        <WorkspacesCard
          onShow={() => send({type: "INVESTIGATION"})}
          kind="investigation"
        />
      </div>
    </Panel>
  );
};

export default WorkspacesHome;
