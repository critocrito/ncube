import React, {useEffect, useState} from "react";

import {listInvestigationSegments} from "../../lib/http";
import {Investigation, Segment, Workspace} from "../../types";
import SegmentCard from "./segment-card";

interface InvestigationDetailsProps {
  workspace: Workspace;
  investigation: Investigation;
  onVerify: (s: Segment) => void;
}

const InvestigationDetails = ({
  workspace: {slug: workspaceSlug},
  investigation,
  onVerify,
}: InvestigationDetailsProps) => {
  const {slug: investigationSlug} = investigation;
  const [segments, setSegments] = useState<Segment[]>([]);

  useEffect(() => {
    const f = async () => {
      const data = await listInvestigationSegments(
        workspaceSlug,
        investigationSlug,
      );
      setSegments(data);
    };
    f();
  }, [workspaceSlug, investigationSlug]);

  return (
    <div>
      <div className="flex items-center pb2">
        <h4 className="header4 sapphire">Segments</h4>
        <div className="ml3 h1 w-100">
          <hr className="sapphire" />
        </div>
      </div>

      <div className="bg-white shadow-4">
        {segments.map((segment) => {
          return (
            <div key={segment.id} className="pv4 ph3">
              <SegmentCard
                investigation={investigation}
                segment={segment}
                onClick={() => onVerify(segment)}
              />
            </div>
          );
        })}
      </div>
    </div>
  );
};

export default InvestigationDetails;
