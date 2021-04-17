import React from "react";

import {voidFn} from "../lib/utils";
import {Investigation, Segment} from "../types";
import InvestigationsSegmentCard from "./investigations-segment-card";

interface InvestigationsDetailsProps {
  investigation: Investigation;
  segments: Segment[];
  onVerify?: (s: Segment) => void;
}

const InvestigationsDetails = ({
  investigation,
  segments,
  onVerify = voidFn,
}: InvestigationsDetailsProps) => {
  return (
    <div>
      <h4 className="header4 border-b border-sapphire py-4 mb-4">Segments</h4>

      <ul className="bg-white space-y-8 p-6">
        {segments.map((segment) => {
          return (
            <li key={segment.id}>
              <InvestigationsSegmentCard
                investigation={investigation}
                segment={segment}
                onVerify={() => onVerify(segment)}
              />
            </li>
          );
        })}
      </ul>
    </div>
  );
};

export default InvestigationsDetails;
