import React from "react";

import {Segment} from "../types";
import SegmentListItem from "./segment-list-item";

interface SegmentListProps {
  segments: Segment[];
  onExplore: (s: Segment) => void;
}

const SegmentList = ({segments, onExplore}: SegmentListProps) => {
  return (
    <div className="flex flex-column">
      <div className="flex">
        <div className="flex flex-column justify-around items-center bg-white br--top br3 w-20 ttu tc h2 sapphire b">
          <span>Segments</span>
        </div>
        <div className="flex flex-column justify-around items-center br--top br3 w-20 ttu tc h2 sapphire b">
          <span>In Verification</span>
        </div>
      </div>

      <ul className="list pl0 bg-white mt0 pt3">
        {segments.map((segment) => {
          return (
            <li key={segment.id} className="ml2 mr2 mt3 mb3">
              <SegmentListItem
                onExplore={() => onExplore(segment)}
                segment={segment}
              />
            </li>
          );
        })}
      </ul>
    </div>
  );
};

export default SegmentList;
