import React from "react";

import {useWorkspaceCtx} from "../lib/context";
import {statSegmentsProgress, statSegmentsVerified} from "../lib/http";
import {Investigation, Segment} from "../types";
import Button from "./button";
import Stat from "./stat";
import Card from "./card";

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
    <Card>
      <table className="w-full h-full max-w-xl">
        <colgroup>
          <col className="w-2/5" />
          <col className="w-1/5" />
          <col className="w-1/5" />
          <col className="w-1/5" />
        </colgroup>

        <thead>
          <tr>
            <th className="border border-fair-pink font-bold text-sapphire p-2">
              <h4 className="header4 text-left">{segment.title}</h4>
            </th>
            <th className="border border-fair-pink text-center text-sapphire">
              New Data
            </th>
            <th className="border border-fair-pink text-center text-sapphire">
              Data In Progress
            </th>
            <th className="border border-fair-pink text-center text-sapphire">
              Verified Data
            </th>
          </tr>
        </thead>

        <tbody>
          <tr>
            <td className="border-l border-b border-r border-fair-pink text-sapphire py-4">
              &nbsp;
            </td>

            <td className="border border-fair-pink text-center text-sapphire">
              &nbsp;
            </td>
            <td className="border border-fair-pink text-center text-sapphire">
              <Stat
                fetchStat={() =>
                  statSegmentsProgress(slug, investigation.slug, segment.slug)
                }
              />
            </td>
            <td className="border border-fair-pink text-center text-sapphire">
              <Stat
                fetchStat={() =>
                  statSegmentsVerified(slug, investigation.slug, segment.slug)
                }
              />
            </td>
          </tr>
        </tbody>
      </table>

      <div className="self-start ml-auto">
        <Button onClick={onVerify}>Verify</Button>
      </div>
    </Card>
  );
};

export default InvestigationsSegmentCard;
