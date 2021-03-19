import React from "react";

import {statSegmentsUnits} from "../../lib/http";
import {Segment, Workspace} from "../../types";
import Button from "../button";
import Stat from "../stat";

interface SegmentItemProps {
  workspace: Workspace;
  segment: Segment;
  onExplore: () => void;
  onVerify: () => void;
  onRemove: () => void;
}

const SegmentListItem = ({
  workspace,
  segment: {query, title, slug},
  onExplore,
  onVerify,
  onRemove,
}: SegmentItemProps) => {
  return (
    <section className="w-100 flex justify-between">
      <div className="flex w-80">
        <div className="w-30 flex flex-column justify-between pb1 pl2 pt1 bt bl bb b--fair-pink">
          <h4 className="header4 ml1 mt1 mb2">{title}</h4>
          <Button
            className="mb2 ml3"
            kind="secondary"
            size="small"
            onClick={onExplore}
          >
            Explore
          </Button>
        </div>

        <div className="w-70 h-100">
          <table className="w-100 h4 collapse bn no-hover">
            <colgroup>
              <col className="w-third" />
              <col className="w-third" />
              <col className="w-third" />
            </colgroup>

            <thead>
              <tr>
                <th className="ba b--fair-pink tc b sapphire">Query</th>
                <th className="ba b--fair-pink tc b sapphire">Units</th>
                <th className="ba b--fair-pink tc b sapphire">New</th>
              </tr>
            </thead>

            <tbody>
              <tr>
                <td className="ba b--fair-pink tc sapphire">{query}</td>
                <td className="ba b--fair-pink tc sapphire">
                  <Stat
                    fetchStat={() => statSegmentsUnits(workspace.slug, slug)}
                  />
                </td>
                <td className="ba b--fair-pink tc sapphire">&mdash;</td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>

      <div className="pr3 h-100 flex flex-column">
        <Button size="small" onClick={onVerify}>
          Verify
        </Button>

        <Button className="mt1" kind="caution" onClick={onRemove}>
          Remove
        </Button>
      </div>
    </section>
  );
};

export default SegmentListItem;
