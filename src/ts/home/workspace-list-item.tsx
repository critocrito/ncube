import React from "react";

import Button from "../common/button";
import WorkspaceTag from "../common/workspace-tag";
import {statDataTotal, statSourcesTotal} from "../http";
import {Workspace} from "../types";
import Stat from "./stat";

interface WorkspaceListItemProps {
  workspace: Workspace;
  handleOpen: () => void;
}

const WorkspaceListItem = ({workspace, handleOpen}: WorkspaceListItemProps) => {
  const {id, kind, name} = workspace;

  return (
    <li key={id} className="bb">
      <div className="flex items-center justify-between w-100">
        <div className="flex flex-wrap w-80">
          <div className=" w-100 flex justify-between items-center">
            <div className="flex w-10 mr3">
              <WorkspaceTag kind={kind} />
            </div>
            <h3 className="header3 flex-nowrap w-100 ma0 pv4">{name}</h3>
          </div>
          <div className="w-70 pb4 mr2">
            <div className="flex items-center justify-between">
              <Stat
                kind="source"
                fetchStat={() => statSourcesTotal(workspace.slug)}
              />
              <Stat
                kind="data"
                fetchStat={() => statDataTotal(workspace.slug)}
              />
              <Stat kind="process" fetchStat={() => Promise.resolve(23)} />
              <Stat
                kind="investigation"
                fetchStat={() => Promise.resolve(23)}
              />
            </div>
          </div>
        </div>

        <Button className="ml1" onClick={handleOpen}>
          Open
        </Button>
      </div>
    </li>
  );
};

export default WorkspaceListItem;
