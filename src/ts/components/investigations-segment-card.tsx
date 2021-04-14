import React from "react";

import {useWorkspaceCtx} from "../lib/context";
import {statSegmentsProgress, statSegmentsVerified} from "../lib/http";
import {Investigation, Segment} from "../types";
import Button from "./button";
import Stat from "./stat";

interface InvestigationsSegmentCardProps {
  investigation: Investigation;
  segment: Segment;
  onVerify: () => void;
}

const InvestigationsSegmentCard = ({
  investigation,
  segment,
  onVerify,
}: InvestigationsSegmentCardProps) => {
  const [
    {
      context: {
        workspace: {slug},
      },
    },
  ] = useWorkspaceCtx();

  return (
    <div className="flex justify-between">
      <div className="w-80">
        <table className="w-100 h-100 collapse bn no-hover">
          <colgroup>
            <col className="w-40" />
            <col className="w-20" />
            <col className="w-20" />
            <col className="w-20" />
          </colgroup>

          <thead>
            <tr>
              <th className="bl br bt b--fair-pink tl b text-sapphire">
                {segment.title}
              </th>
              <th className="ba b--fair-pink tc b text-sapphire">New Data</th>
              <th className="ba b--fair-pink tc b text-sapphire">
                Data In Progress
              </th>
              <th className="ba b--fair-pink tc b text-sapphire">
                Verified Data
              </th>
            </tr>
          </thead>

          <tbody>
            <tr>
              <td className="bl br bb b--fair-pink tl text-sapphire">&nbsp;</td>
              <td className="ba b--fair-pink tc text-sapphire">&mdash;</td>
              <td className="ba b--fair-pink tc text-sapphire">
                <Stat
                  fetchStat={() =>
                    statSegmentsProgress(slug, investigation.slug, segment.slug)
                  }
                />
              </td>
              <td className="ba b--fair-pink tc text-sapphire">
                <Stat
                  fetchStat={() =>
                    statSegmentsVerified(slug, investigation.slug, segment.slug)
                  }
                />
              </td>
            </tr>
          </tbody>
        </table>
      </div>

      <div>
        <Button onClick={onVerify} size="large">
          Verify
        </Button>
      </div>
    </div>
  );
};

export default InvestigationsSegmentCard;
