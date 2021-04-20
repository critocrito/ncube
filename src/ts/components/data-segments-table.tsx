import React, {useState} from "react";

import {Segment, Workspace} from "../types";
import DataSegmentsCard from "./data-segments-card";
import Tabs from "./tabs";

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
  const items = [
    {label: "Segments", value: "segments"},
    // {label: "In Verification", value: "verification"},
  ];

  const [selected, setSelected] = useState(items[0]);

  return (
    <div className="flex flex-col">
      <Tabs items={items} selected={selected} onClick={setSelected} />

      <ul className="bg-white space-y-8 p-6">
        {segments.map((segment) => {
          return (
            <li key={segment.id}>
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
