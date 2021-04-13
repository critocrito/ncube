import React from "react";

import DeleteWorkspace from "../forms/delete-workspace";
import {deleteWorkspace} from "../lib/http";
import {DeleteEventYes} from "../machines/delete";
import {Workspace} from "../types";
import ConfirmDelete from "./confirm-delete";

interface DashboardDeleteWorkspaceProps {
  workspace: Workspace;
  onDone: () => void;
}

const DashboardDeleteWorkspace = ({
  workspace,
  onDone,
}: DashboardDeleteWorkspaceProps) => {
  const {name, description, location} = workspace;

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
          <div className="flex flex-column">
            <h3 className="header3">
              Are you sure you want to delete this workspace?
            </h3>

            <p className="mb2 b sapphire">Name</p>
            <div className="flex items-start justify-between">
              <span className="w-90">{name}</span>
            </div>

            {description && (
              <>
                <p className="mb2 b sapphire">Description</p>
                <div className="flex items-start justify-between">
                  <span className="w-90">{description}</span>
                </div>
              </>
            )}

            <p className="mb2 b sapphire">Location</p>
            <div className="flex items-start justify-between">
              <span className="w-90">{location}</span>
            </div>

            <div className="mt4">
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
