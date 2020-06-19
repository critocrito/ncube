import React from "react";

import {Workspace} from "../../types";
import Button from "../base/button";
import WorkspaceTag from "../base/workspace-tag";
import Stat from "./stat";

interface WorkspaceListItemProps {
  workspace: Workspace;
  stats: {
    query: number;
    data: number;
    process: number;
    investigation: number;
  };
  handleOpen: () => void;
}

const WorkspaceListItem = ({
  workspace,
  handleOpen,
  stats,
}: WorkspaceListItemProps) => {
  const {id, kind, name} = workspace;

  return (
    <li key={id} className="bb">
      <div className="flex items-center justify-between w-100">
        <h3 className="header3 nowrap sapphire w-30">{name}</h3>
        <div className="flex justify-between items-center w-70">
          <WorkspaceTag kind={kind} />
          <div className="flex items-center justify-between">
            <Stat kind="query" value={stats.query} />
            <Stat kind="data" value={stats.data} />
            <Stat kind="process" value={stats.process} />
            <Stat kind="investigation" value={stats.investigation} />
          </div>
          <Button className="ml1" onClick={handleOpen}>
            Open
          </Button>
        </div>
      </div>
    </li>
  );
};

export default WorkspaceListItem;
