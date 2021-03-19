import React, {useState} from "react";

import InvestigationsIntroduction from "../../mdx/investigations-intro.mdx";
import IntroText from "../components/intro-text";
import Investigation from "../components/investigation";
import Panel from "../components/panel";
import {useAppCtx, useWorkspaceCtx} from "../lib/context";

const Investigations = () => {
  const [investigationsHeader, setInvestigationsHeader] = useState(
    "Investigations",
  );
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
      header={investigationsHeader}
      description=""
    >
      <>
        <IntroText>
          <InvestigationsIntroduction />
        </IntroText>

        <Investigation
          workspace={workspace}
          onHeaderChange={(title) =>
            setInvestigationsHeader(
              title ? `Investigations: ${title}` : "Investigations",
            )
          }
        />
      </>
    </Panel>
  );
};

export default Investigations;
