import React from "react";

import WorkspacesEmpty from "../../mdx/workspaces-empty.mdx";
import IntroText from "../common/intro-text";
import Overline from "../common/overline";
import {Workspace} from "../types";
import WorkspaceListItem from "./workspace-list-item";

interface WorkspaceListProps {
  workspaces: Workspace[];
  workspaceAction: (action: "show" | "delete", workspace: Workspace) => void;
}

const WorkspaceList = ({workspaces, workspaceAction}: WorkspaceListProps) => {
  return workspaces.length > 0 ? (
    <>
      <Overline className="pt4" label="Workspaces" />

      <ul className="list pl0 mt0 mb0">
        {workspaces.map((workspace) => (
          <WorkspaceListItem
            key={workspace.id}
            workspace={workspace}
            handleOpen={() => workspaceAction("show", workspace)}
            handleRemove={() => workspaceAction("delete", workspace)}
          />
        ))}
      </ul>
    </>
  ) : (
    <IntroText>
      <WorkspacesEmpty />
    </IntroText>
  );
};

export default WorkspaceList;
