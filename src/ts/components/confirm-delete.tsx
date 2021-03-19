import React from "react";

import DeleteWorkspace from "../forms/delete-workspace";
import {Workspace} from "../types";

interface ConfirmDeleteProps {
  workspace: Workspace;
  onCancel: () => void;
  onDelete: (removeLocation: boolean) => void;
}

const ConfirmDelete = ({
  workspace: {name, description, location},
  onCancel,
  onDelete,
}: ConfirmDeleteProps) => {
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
          onSubmit={({delete_location: deleteLocation}) =>
            onDelete(deleteLocation)
          }
        />
      </div>
    </div>
  );
};

export default ConfirmDelete;
