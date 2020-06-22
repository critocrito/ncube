import {RouteComponentProps} from "@reach/router";
import {useMachine} from "@xstate/react";
import React from "react";

import Panel from "../layout/panel";
import SectionCard from "../workspace/section-card";
import {list, show} from "../http/workspace";
import WorkspaceMachine from "../machines/workspace";
import {unreachable} from "../utils";

interface WorkspaceProps extends RouteComponentProps {
  slug?: string;
}

const Workspace = ({slug}: WorkspaceProps) => {
  // FIXME: I need to handle this error better.
  if (slug === undefined) {
    throw new Error("Boom!");
  }
  // FIXME: Remove this lint ignore
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  const [state, send] = useMachine(WorkspaceMachine, {
    services: {
      prepareData: async (_ctx, _ev) => {
        const [workspaces, current] = await Promise.all([list(), show(slug)]);
        return {workspaces, current};
      },
    },
  });

  switch (true) {
    case state.matches("overview"):
      // FIXME: Can I get by without this check? In any case I need to handle that error more smoothly.
      if (state.context.current === undefined) {
        return <p />;
      }

      return (
        <Panel
          workspaces={state.context.workspaces}
          workspace={state.context.current}
        >
          <div>
            <SectionCard kind="source" />
            <SectionCard kind="data" />
            <SectionCard kind="process" />
            <SectionCard kind="investigation" />
          </div>
        </Panel>
      );

    case state.matches("prepareData"):
      return <p>Loading</p>;

    default:
      return unreachable("Workspace route didn't match any valid state.");
  }
};

export default Workspace;
