import c from "clsx";
import React, {useEffect, useState} from "react";

import WorkspacesEmpty from "../../mdx/workspaces-empty.mdx";
import {useNcubeCtx} from "../lib/context";
import {
  statDataTotal,
  statInvestigationsTotal,
  statSourcesTotal,
} from "../lib/http";
import {Notification, Workspace} from "../types";
import Button from "./button";
import IntroText from "./intro-text";
import LoadingSpinner from "./loading-spinner";
import Overline from "./overline";
import Stat from "./stat-dashboard";
import WorkspaceTag from "./workspace-tag";

interface DashboardWorkspacesProps {
  workspaces: Workspace[];
  onShow: (workspace: Workspace) => void;
  onDelete: (workspace: Workspace) => void;
}

interface DashboardWorkspaceItemProps {
  workspace: Workspace;
  handleOpen: () => void;
  handleRemove: () => void;
}

const extractMessage = (msg: Notification | void): string | undefined => {
  if (msg === undefined) return;

  // eslint-disable-next-line default-case
  switch (msg.kind) {
    case "queued": {
      // eslint-disable-next-line consistent-return
      return "Waiting to start workspace creation.";
    }
    case "start": {
      // eslint-disable-next-line consistent-return
      return "Started to create the workspace.";
    }
    case "progress": {
      const {
        data: {msg: message},
      } = msg;
      // eslint-disable-next-line consistent-return
      return message;
    }
    case "done": {
      // eslint-disable-next-line consistent-return
      return "Workspace successfully created.";
    }
    case "error": {
      const {
        data: {error: message},
      } = msg;
      // eslint-disable-next-line consistent-return
      return `Error creating workspace: ${message}`;
    }
  }
};

const DashboardWorkspaceItem = ({
  workspace,
  handleOpen,
  handleRemove,
}: DashboardWorkspaceItemProps) => {
  const {kind, name, slug} = workspace;
  const topic = `task.${slug}.setup_workspace`;
  const [isCreated, setIsCreated] = useState(workspace.is_created);
  const [isError, setIsError] = useState(false);

  const [
    {
      context: {pubsub},
    },
  ] = useNcubeCtx();

  const [message, setMessage] = useState<string | undefined>(
    extractMessage(pubsub.lastMessage(topic)),
  );

  useEffect(() => {
    if (isCreated) return;

    const unsubscribe = pubsub.subscribe(topic, (msg: Notification) => {
      setMessage(extractMessage(msg));
      if (msg.kind === "done") {
        setIsCreated(true);
        pubsub.finish(topic);
      }
      if (msg.kind === "error") {
        setIsError(true);
        pubsub.finish(topic);
      }
    });

    // eslint-disable-next-line consistent-return
    return unsubscribe;
  });

  return (
    <li className="bb">
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
          <div className="flex">
            {kind === "local" && (
              <Button className="ml1" kind="caution" onClick={handleRemove}>
                Remove
              </Button>
            )}
            <Button className="ml1" onClick={handleOpen}>
              Open
            </Button>
          </div>
        ) : (
          <div className="flex flex-column">
            <LoadingSpinner className="ml-auto" />
            {message && (
              <div
                className={c(
                  "pt2 text-md ml-auto nowrap",
                  isError ? "error" : "success",
                )}
              >
                {message}
              </div>
            )}
          </div>
        )}
      </div>
    </li>
  );
};

const DashboardWorkspaces = ({
  workspaces,
  onShow,
  onDelete,
}: DashboardWorkspacesProps) => {
  return workspaces.length > 0 ? (
    <>
      <Overline className="pt4" label="Workspaces" />

      <ul className="list pl0 mt0 mb0">
        {workspaces.map((workspace) => (
          <DashboardWorkspaceItem
            key={workspace.id}
            workspace={workspace}
            handleOpen={() => onShow(workspace)}
            handleRemove={() => onDelete(workspace)}
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

export default DashboardWorkspaces;
