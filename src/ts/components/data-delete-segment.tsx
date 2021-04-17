import React from "react";

import DeleteSegment from "../forms/delete-segment";
import {deleteSegment} from "../lib/http";
import {Segment, Workspace} from "../types";
import ConfirmDelete from "./confirm-delete";
import Description from "./description";

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

  const items = [
    {label: "Title", value: title},
    {label: "Query", value: query},
  ];

  return (
    <ConfirmDelete<Record<string, unknown>>
      onDelete={async (): Promise<void> => {
        await deleteSegment(workspace.slug, segment.slug);
      }}
      onDone={onDone}
    >
      {({onSubmit, onCancel}) => {
        return (
          <div className="flex flex-col">
            <h4 className="header4 mb-3">
              Are you sure you want to delete this Segment?
            </h4>

            <Description items={items} />

            <div className="mt-3">
              <DeleteSegment onCancel={onCancel} onSubmit={onSubmit} />
            </div>
          </div>
        );
      }}
    </ConfirmDelete>
  );
};

export default DataDeleteSegment;
