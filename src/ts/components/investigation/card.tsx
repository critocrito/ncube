import React from "react";

import {useWorkspaceCtx} from "../../lib/context";
import {
  statInvestigationsData,
  statInvestigationsSegments,
  statInvestigationsVerified,
} from "../../lib/http";
import {Investigation} from "../../types";
import Button from "../button";
import Stat from "../stat";

interface InvestigationCardProps {
  investigation: Investigation;
  onClick: () => void;
}

const InvestigationCard = ({
  investigation: {title, slug: investigationSlug},
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
    <section className="bg-white pa3 shadow-4 flex items-center justify-between mb4">
      <div className="flex w-80 h-100">
        <table className="w-100 h-100 collapse bn card">
          <colgroup>
            <col className="w-40" />
            <col className="w-20" />
            <col className="w-20" />
            <col className="w-20" />
          </colgroup>

          <thead>
            <tr>
              <th className="bl bt br b--fair-pink tl b sapphire">
                <h4 className="header4 mt0 mb0">{title}</h4>
              </th>
              <th className="ba b--fair-pink tc b sapphire">All</th>
              <th className="ba b--fair-pink tc b sapphire">Segments</th>
              <th className="ba b--fair-pink tc b sapphire">Verified</th>
            </tr>
          </thead>

          <tbody>
            <tr>
              <td className="bl br bb b--fair-pink tc sapphire">&nbsp;</td>

              <td className="ba b--fair-pink tc sapphire">
                <Stat
                  fetchStat={() =>
                    statInvestigationsData(slug, investigationSlug)
                  }
                />
              </td>
              <td className="ba b--fair-pink tc sapphire">
                <Stat
                  fetchStat={() =>
                    statInvestigationsSegments(slug, investigationSlug)
                  }
                />
              </td>
              <td className="ba b--fair-pink tc sapphire">
                <Stat
                  fetchStat={() =>
                    statInvestigationsVerified(slug, investigationSlug)
                  }
                />
              </td>
            </tr>
          </tbody>
        </table>
      </div>

      <div className="pr3 h-100 flex flex-column">
        <Button onClick={onClick}>Open</Button>
      </div>
    </section>
  );
};

export default InvestigationCard;
