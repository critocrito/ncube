import React from "react";

import Button from "../common/button";
import Stat from "../common/stat";
import {useWorkspaceCtx} from "../context";
import {
  statInvestigationsData,
  statInvestigationsSegments,
  statInvestigationsVerified,
} from "../http";
import {Investigation} from "../types";

interface InvestigationCardProps {
  investigation: Investigation;
  onClick: () => void;
}

const InvestigationCard = ({
  investigation: {title, slug: investigation_slug},
  onClick,
}: InvestigationCardProps) => {
  const [
    {
      context: {
        workspace: {slug},
      },
    },
  ] = useWorkspaceCtx();

  return (
    <section className="h4 bg-white pa3 shadow-4 flex items-center justify-between mb4">
      <div className="flex w-80">
        <table className="w-100 collapse bn card">
          <colgroup>
            <col className="w-40" />
            <col className="w-20" />
            <col className="w-20" />
            <col className="w-20" />
          </colgroup>

          <tr>
            <th className="ba b--fair-pink tc b sapphire tl v-top" rowSpan={2}>
              <h4 className="header4 mt0 mb0 tl">{title}</h4>
            </th>
            <th className="ba b--fair-pink tc b sapphire">All</th>
            <th className="ba b--fair-pink tc b sapphire">Segments</th>
            <th className="ba b--fair-pink tc b sapphire">Verified</th>
          </tr>

          <tr>
            <td className="ba b--fair-pink tc sapphire">
              <Stat
                fetchStat={() =>
                  statInvestigationsData(slug, investigation_slug)
                }
              />
            </td>
            <td className="ba b--fair-pink tc sapphire">
              <Stat
                fetchStat={() =>
                  statInvestigationsSegments(slug, investigation_slug)
                }
              />
            </td>
            <td className="ba b--fair-pink tc sapphire">
              <Stat
                fetchStat={() =>
                  statInvestigationsVerified(slug, investigation_slug)
                }
              />
            </td>
          </tr>
        </table>
      </div>

      <div className="pr3 h-100 flex flex-column">
        <Button onClick={onClick}>Open</Button>
      </div>
    </section>
  );
};

export default InvestigationCard;
