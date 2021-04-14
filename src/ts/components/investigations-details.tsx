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
      <div className="flex items-center pb2">
        <h4 className="header4">Segments</h4>
        <div className="ml3 h1 w-100">
          <hr className="text-sapphire" />
        </div>
      </div>

      <div className="bg-white shadow-4">
        {segments.map((segment) => {
          return (
            <div key={segment.id} className="pv4 ph3">
              <InvestigationsSegmentCard
                investigation={investigation}
                segment={segment}
                onVerify={() => onVerify(segment)}
              />
            </div>
          );
        })}
      </div>
    </div>
  );
};

export default InvestigationsDetails;
