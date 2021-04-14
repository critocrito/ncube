import React from "react";

import DeleteSource from "../forms/delete-source";
import {removeSource} from "../lib/http";
import {Source, Workspace} from "../types";
import ConfirmDelete from "./confirm-delete";

interface SourcesDeleteProps {
  workspace: Workspace;
  source: Source;
  onDone: () => void;
}

const SourcesDelete = ({workspace, source, onDone}: SourcesDeleteProps) => {
  const {type, term} = source;

  return (
    <ConfirmDelete<Record<string, unknown>>
      onDelete={async (): Promise<void> => {
        await removeSource(workspace.slug, source.id);
      }}
      onDone={onDone}
    >
      {({onSubmit, onCancel}) => {
        return (
          <div className="flex flex-column">
            <h3 className="header3">
              Are you sure you want to delete this source?
            </h3>

            <p className="mb2 b text-sapphire">Title</p>
            <div className="flex items-start justify-between">
              <dl className="pa4 mt0 text-sapphire">
                <dt className="f6 b">Type</dt>
                <dd className="ml0">{type}</dd>
                <dt className="f6 b mt2">Term</dt>
                <dd className="ml0">{term}</dd>
              </dl>
            </div>

            <div className="mt4">
              <DeleteSource onCancel={onCancel} onSubmit={onSubmit} />
            </div>
          </div>
        );
      }}
    </ConfirmDelete>
  );
};

export default SourcesDelete;
