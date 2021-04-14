import React from "react";

import {Segment, Workspace} from "../types";
import DataSegmentsCard from "./data-segments-card";

interface DataSegmentsTableProps {
  workspace: Workspace;
  segments: Segment[];
  onShow: (s: Segment) => void;
  onVerify: (s: Segment) => void;
  onDelete: (s: Segment) => void;
}

const DataSegmentsTable = ({
  segments,
  workspace,
  onShow,
  onVerify,
  onDelete,
}: DataSegmentsTableProps) => {
  return (
    <div className="flex flex-column">
      <div className="flex">
        <div className="flex flex-column justify-around items-center bg-white br--top br3 w-20 ttu tc h2 text-sapphire b">
          <span>Segments</span>
        </div>
        <div className="flex flex-column justify-around items-center br--top br3 w-20 ttu tc h2 text-sapphire b">
          <span>In Verification</span>
        </div>
      </div>

      <ul className="list pl0 bg-white mt0 pt3">
        {segments.map((segment) => {
          return (
            <li key={segment.id} className="ml2 mr2 mt3 mb3">
              <DataSegmentsCard
                workspace={workspace}
                onShow={() => onShow(segment)}
                onVerify={() => onVerify(segment)}
                onDelete={() => onDelete(segment)}
                segment={segment}
              />
            </li>
          );
        })}
      </ul>
    </div>
  );
};

export default DataSegmentsTable;
