import React from "react";

import ProcessesIntroduction from "../../mdx/processes-intro.mdx";
import IntroText from "../components/intro-text";
import Panel from "../components/panel";
import Process from "../components/processes";
import {useAppCtx, useWorkspaceCtx} from "../lib/context";

const Processes = () => {
  const [
    {
      context: {workspaces},
    },
  ] = useAppCtx();
  const [
    {
      context: {workspace},
    },
  ] = useWorkspaceCtx();

  return (
    <Panel
      workspaces={workspaces}
      workspace={workspace}
      header="Processes"
      description=""
    >
      <>
        <IntroText>
          <ProcessesIntroduction />
        </IntroText>

        <Process workspace={workspace} />
      </>
    </Panel>
  );
};

export default Processes;
