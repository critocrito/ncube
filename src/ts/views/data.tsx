import React, {useState} from "react";

import DataIntroduction from "../../mdx/data-intro.mdx";
import Database from "../components/data";
import IntroText from "../components/intro-text";
import Panel from "../components/panel";
import {useAppCtx, useWorkspaceCtx} from "../lib/context";

const Data = () => {
  const [databaseHeader, setDatabaseHeader] = useState("Database");
  const [
    {
      context: {workspaces},
    },
  ] = useAppCtx();
  const [
    {
      context: {workspace, dataStats},
    },
  ] = useWorkspaceCtx();

  return (
    <Panel
      workspaces={workspaces}
      workspace={workspace}
      header={databaseHeader}
      description=""
    >
      <>
        <IntroText>
          <DataIntroduction />
        </IntroText>

        <Database
          stats={dataStats}
          workspace={workspace}
          onHeaderChange={(title) =>
            setDatabaseHeader(title ? `Database: ${title}` : "Database")
          }
        />
      </>
    </Panel>
  );
};

export default Data;
