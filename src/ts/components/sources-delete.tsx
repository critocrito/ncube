import React from "react";

import DeleteSource from "../forms/delete-source";
import {removeSource} from "../lib/http";
import {Source, Workspace} from "../types";
import ConfirmDelete from "./confirm-delete";
import Description from "./description";

interface SourcesDeleteProps {
  workspace: Workspace;
  source: Source;
  onDone: () => void;
}

const SourcesDelete = ({workspace, source, onDone}: SourcesDeleteProps) => {
  const {type, term} = source;

  const items = [
    {label: "Type", value: type},
    {label: "Term", value: term},
  ];

  return (
    <ConfirmDelete<Record<string, unknown>>
      onDelete={async (): Promise<void> => {
        await removeSource(workspace.slug, source.id);
      }}
      onDone={onDone}
    >
      {({onSubmit, onCancel}) => {
        return (
          <div className="flex flex-col">
            <h3 className="header3">
              Are you sure you want to delete this source?
            </h3>

            <Description items={items} />

            <div className="mt-">
              <DeleteSource onCancel={onCancel} onSubmit={onSubmit} />
            </div>
          </div>
        );
      }}
    </ConfirmDelete>
  );
};

export default SourcesDelete;
