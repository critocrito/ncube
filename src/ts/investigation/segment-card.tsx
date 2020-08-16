import React from "react";

import Button from "../common/button";
import Stat from "../common/stat";
import {useWorkspaceCtx} from "../context";
import {statSegmentsProgress, statSegmentsVerified} from "../http";
import {Investigation, Segment} from "../types";

interface SegmentCardProps {
  investigation: Investigation;
  segment: Segment;
  onClick: () => void;
}

const SegmentCard = ({investigation, segment, onClick}: SegmentCardProps) => {
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
              <th className="bl br bt b--fair-pink tl b sapphire">
                {segment.title}
              </th>
              <th className="ba b--fair-pink tc b sapphire">New Data</th>
              <th className="ba b--fair-pink tc b sapphire">
                Data In Progress
              </th>
              <th className="ba b--fair-pink tc b sapphire">Verified Data</th>
            </tr>
          </thead>

          <tbody>
            <tr>
              <td className="bl br bb b--fair-pink tl sapphire">&nbsp;</td>
              <td className="ba b--fair-pink tc sapphire">&mdash;</td>
              <td className="ba b--fair-pink tc sapphire">
                <Stat
                  fetchStat={() =>
                    statSegmentsProgress(slug, investigation.slug, segment.slug)
                  }
                />
              </td>
              <td className="ba b--fair-pink tc sapphire">
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
        <Button onClick={onClick} size="large">
          Verify
        </Button>
      </div>
    </div>
  );
};

export default SegmentCard;
