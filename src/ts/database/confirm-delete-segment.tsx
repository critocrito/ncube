import React from "react";

import DeleteSegment from "../forms/delete-segment";
import {Segment} from "../types";

interface ConfirmDeleteSegmentProps {
  segment: Segment;
  onCancel: () => void;
  onDelete: () => void;
}

const ConfirmDeleteSegment = ({
  segment: {title, query},
  onCancel,
  onDelete,
}: ConfirmDeleteSegmentProps) => {
  return (
    <div className="flex flex-column">
      <h3 className="header3">Are you sure you want to delete this Segment?</h3>

      <p className="mb2 b sapphire">Title</p>
      <div className="flex items-start justify-between">
        <span className="w-90">{title}</span>
      </div>

      <p className="mb2 b sapphire">Query</p>
      <div className="flex items-start justify-between">
        <span className="w-90">{query}</span>
      </div>

      <div className="mt4">
        <DeleteSegment onCancel={onCancel} onSubmit={onDelete} />
      </div>
    </div>
  );
};

export default ConfirmDeleteSegment;
