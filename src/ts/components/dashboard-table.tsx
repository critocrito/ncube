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
import DashboardStats from "./dashboard-stats";
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
    <li className="border-b">
      <div className="flex items-center justify-between">
        <div className="flex flex-col justify-around h-32 my-1 md:my-4">
          <div className="flex items-center space-x-4">
            <WorkspaceTag kind={kind} />

            <h3 className="header3">{name}</h3>
          </div>

          {isCreated ? (
            <DashboardStats
              fetchSourcesStat={() => statSourcesTotal(workspace.slug)}
              fetchDataStat={() => statDataTotal(workspace.slug)}
              fetchInvestigationsStat={() =>
                statInvestigationsTotal(workspace.slug)
              }
            />
          ) : (
            <p className="max-w-lg">
              This workspace is being created in the background. Depending on
              your computer and Internet speed this can take some time.
            </p>
          )}
        </div>

        {isCreated ? (
          <div className="flex flex-col md:items-center md:flex-row space-y-4 md:space-y-0 md:space-x-4">
            {kind === "local" && (
              <Button kind="caution" onClick={handleRemove}>
                Remove
              </Button>
            )}
            <Button onClick={handleOpen}>Open</Button>
          </div>
        ) : (
          <div className="flex flex-col items-end">
            <LoadingSpinner />
            {message && (
              <span
                className={c("pt-2 text-sm whitespace-nowrap", {
                  "text-error": isError,
                  "text-success": !isError,
                })}
              >
                {message}
              </span>
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
  // className="list-inside pl0 mt0 mb0"
  return workspaces.length > 0 ? (
    <>
      <Overline className="pt-4" label="Workspaces" />

      <ul>
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
