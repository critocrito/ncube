import React, {useEffect, useState} from "react";

import Button from "../common/button";
import Stat from "../common/stat";
import {useWorkspaceCtx} from "../context";
import {statInvestigationsSegments} from "../http";
import {Investigation} from "../types";

interface InvestigationCardProps {
  investigation: Investigation;
  onClick: () => void;
}

const InvestigationCard = ({
  investigation: {title, id, slug: investigation_slug},
  onClick,
}: InvestigationCardProps) => {
  const [investigationsAll, setInvestigationsAll] = useState(0);

  const [
    {
      context: {
        workspace: {slug},
      },
    },
  ] = useWorkspaceCtx();

  useEffect(() => {
    const f = async () => {
      // const stat = await statInvestigationsAll(slug, key);
      const stat = 23;
      setInvestigationsAll(stat);
    };
    f();
  }, [slug, id]);

  return (
    <section className="h4 bg-white pa3 shadow-4 flex items-center justify-between mb4">
      <div className="flex w-80 h-100">
        <div className="w-40 h-100 flex flex-column justify-between pa2 bt bl bb b--fair-pink">
          <h4 className="header4 mt0 mb0">{title}</h4>
        </div>

        <div className="w-60 h-100">
          <table className="w-100 h-100 collapse bn no-hover">
            <colgroup>
              <col className="w-third" />
              <col className="w-third" />
              <col className="w-third" />
            </colgroup>

            <thead>
              <tr>
                <th className="ba b--fair-pink tc b sapphire">All</th>
                <th className="ba b--fair-pink tc b sapphire">Segments</th>
                <th className="ba b--fair-pink tc b sapphire">Verified</th>
              </tr>
            </thead>

            <tbody>
              <tr>
                <td className="ba b--fair-pink tc sapphire">
                  {investigationsAll === 0 ? (
                    <>&mdash;</>
                  ) : (
                    `${investigationsAll} units`
                  )}
                </td>
                <td className="ba b--fair-pink tc sapphire">
                  <Stat
                    fetchStat={() =>
                      statInvestigationsSegments(slug, investigation_slug)
                    }
                  />
                </td>
                <td className="ba b--fair-pink tc sapphire">&mdash;</td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>

      <div className="pr3 h-100 flex flex-column">
        <Button onClick={onClick}>Open</Button>
      </div>
    </section>
  );
};

export default InvestigationCard;
