import React from "react";

import Button from "../common/button";
import LoadingSpinner from "../common/loading-spinner";
import WorkspaceTag from "../common/workspace-tag";
import {
  statDataTotal,
  statInvestigationsTotal,
  statSourcesTotal,
} from "../http";
import {Workspace} from "../types";
import Stat from "./stat";

interface WorkspaceListItemProps {
  workspace: Workspace;
  handleOpen: () => void;
}

const WorkspaceListItem = ({workspace, handleOpen}: WorkspaceListItemProps) => {
  const {id, kind, name, is_created: isCreated} = workspace;

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
            {isCreated ? (
              <div className="flex items-center justify-between">
                <Stat
                  kind="source"
                  fetchStat={() => statSourcesTotal(workspace.slug)}
                />
                <Stat
                  kind="data"
                  fetchStat={() => statDataTotal(workspace.slug)}
                />
                <Stat
                  kind="investigation"
                  fetchStat={() => statInvestigationsTotal(workspace.slug)}
                />
              </div>
            ) : (
              <div>
                This workspace is being created in the background. Depending on
                your computer and Internet speed this can take some time.
              </div>
            )}
          </div>
        </div>

        {isCreated ? (
          <Button className="ml1" onClick={handleOpen}>
            Open
          </Button>
        ) : (
          <LoadingSpinner />
        )}
      </div>
    </li>
  );
};

export default WorkspaceListItem;
