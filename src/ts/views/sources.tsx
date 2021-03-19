import React from "react";

import SourcesIntroduction from "../../mdx/sources-intro.mdx";
import IntroText from "../components/intro-text";
import Panel from "../components/panel";
import SourcesTable from "../components/sources/sources-table";
import {useAppCtx, useWorkspaceCtx} from "../lib/context";

const Sources = () => {
  const [
    {
      context: {workspaces},
    },
  ] = useAppCtx();
  const [
    {
      context: {workspace, sourceStats},
    },
  ] = useWorkspaceCtx();

  return (
    <Panel
      workspaces={workspaces}
      workspace={workspace}
      header="Sources"
      description=""
    >
      <>
        <IntroText>
          <SourcesIntroduction />
        </IntroText>

        <SourcesTable workspace={workspace} totalStat={sourceStats.total} />
      </>
    </Panel>
  );
};

export default Sources;
