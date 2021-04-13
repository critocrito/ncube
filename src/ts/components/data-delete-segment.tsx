import React from "react";

import DeleteSegment from "../forms/delete-segment";
import {deleteSegment} from "../lib/http";
import {Segment, Workspace} from "../types";
import ConfirmDelete from "./confirm-delete";

interface DataDeleteSegmentProps {
  workspace: Workspace;
  segment: Segment;
  onDone: () => void;
}

const DataDeleteSegment = ({
  workspace,
  segment,
  onDone,
}: DataDeleteSegmentProps) => {
  const {title, query} = segment;

  return (
    <ConfirmDelete<Record<string, unknown>>
      onDelete={async (): Promise<void> => {
        await deleteSegment(workspace.slug, segment.slug);
      }}
      onDone={onDone}
    >
      {({onSubmit, onCancel}) => {
        return (
          <div className="flex flex-column">
            <h3 className="header3">
              Are you sure you want to delete this Segment?
            </h3>

            <p className="mb2 b sapphire">Title</p>
            <div className="flex items-start justify-between">
              <span className="w-90">{title}</span>
            </div>

            <p className="mb2 b sapphire">Query</p>
            <div className="flex items-start justify-between">
              <span className="w-90">{query}</span>
            </div>

            <div className="mt4">
              <DeleteSegment onCancel={onCancel} onSubmit={onSubmit} />
            </div>
          </div>
        );
      }}
    </ConfirmDelete>
  );
};

export default DataDeleteSegment;
