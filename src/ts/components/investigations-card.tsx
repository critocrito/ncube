import React from "react";

import {useWorkspaceCtx} from "../lib/context";
import {
  statInvestigationsData,
  statInvestigationsSegments,
  statInvestigationsVerified,
} from "../lib/http";
import {Investigation} from "../types";
import Button from "./button";
import Stat from "./stat";
import Card from "./card";

interface InvestigationsCardProps {
  investigation: Investigation;
  onShow: () => void;
}

const InvestigationsCard = ({
  investigation: {title, slug: investigationSlug},
  onShow,
}: InvestigationsCardProps) => {
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
              <h4 className="header4 text-left">{title}</h4>
            </th>
            <th className="border border-fair-pink text-center text-sapphire">
              All
            </th>
            <th className="border border-fair-pink text-center text-sapphire">
              Segments
            </th>
            <th className="border border-fair-pink text-center text-sapphire">
              Verified
            </th>
          </tr>
        </thead>

        <tbody>
          <tr>
            <td className="border-l border-b border-r border-fair-pink text-sapphire py-4">
              &nbsp;
            </td>

            <td className="border border-fair-pink text-center text-sapphire">
              <Stat
                fetchStat={() =>
                  statInvestigationsData(slug, investigationSlug)
                }
              />
            </td>
            <td className="border border-fair-pink text-center text-sapphire">
              <Stat
                fetchStat={() =>
                  statInvestigationsSegments(slug, investigationSlug)
                }
              />
            </td>
            <td className="border border-fair-pink text-center text-sapphire">
              <Stat
                fetchStat={() =>
                  statInvestigationsVerified(slug, investigationSlug)
                }
              />
            </td>
          </tr>
        </tbody>
      </table>

      <div className="self-start ml-auto">
        <Button onClick={onShow}>Open</Button>
      </div>
    </Card>
  );
};

export default InvestigationsCard;
