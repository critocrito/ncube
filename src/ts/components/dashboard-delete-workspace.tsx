import React from "react";

import DeleteWorkspace from "../forms/delete-workspace";
import {deleteWorkspace} from "../lib/http";
import {DeleteEventYes} from "../machines/delete";
import {Workspace} from "../types";
import ConfirmDelete from "./confirm-delete";
import Description from "./description";

interface DashboardDeleteWorkspaceProps {
  workspace: Workspace;
  onDone: () => void;
}

const DashboardDeleteWorkspace = ({
  workspace,
  onDone,
}: DashboardDeleteWorkspaceProps) => {
  const {name, description, location} = workspace;

  const items = [
    {label: "Name", value: name},
    {label: "Description", value: description},
    {label: "Location", value: location},
  ];
  return (
    <ConfirmDelete<{removeLocation: boolean}>
      onDelete={async (_ctx, ev): Promise<void> => {
        const {
          data: {removeLocation},
        } = ev as DeleteEventYes<{
          removeLocation: boolean;
        }>;
        await deleteWorkspace(workspace.slug, removeLocation);
      }}
      onDone={onDone}
    >
      {({onSubmit, onCancel}) => {
        return (
          <div>
            <div className="pb-5">
              <h3 className="header3">
                Are you sure you want to delete this workspace?
              </h3>
              <p className="mt-2 max-w-2xl">
                Deleting a workspace will permanently remove this workspace from
                Ncube. Selecting "Yes" to delete the workspace location will
                delete the workspace directory and all downloads as well.
              </p>
            </div>

            <Description items={items} />

            <div className="mt-3">
              <DeleteWorkspace
                onCancel={onCancel}
                onSubmit={({delete_location: removeLocation}) =>
                  onSubmit({removeLocation})
                }
              />
            </div>
          </div>
        );
      }}
    </ConfirmDelete>
  );
};

export default DashboardDeleteWorkspace;
