import React from "react";

import Button from "../common/button";
import {Segment} from "../types";

interface SegmentCardProps {
  segment: Segment;
  onClick: () => void;
}

const SegmentCard = ({segment, onClick}: SegmentCardProps) => {
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
              <th className="ba b--fair-pink tl b sapphire">{segment.title}</th>
              <th className="ba b--fair-pink tc b sapphire">New Data</th>
              <th className="ba b--fair-pink tc b sapphire">Data In Process</th>
              <th className="ba b--fair-pink tc b sapphire">Verified Data</th>
            </tr>
          </thead>

          <tbody>
            <tr>
              <td className="ba b--fair-pink tl sapphire">&nbsp;</td>
              <td className="ba b--fair-pink tc sapphire">&mdash;</td>
              <td className="ba b--fair-pink tc sapphire">&mdash;</td>
              <td className="ba b--fair-pink tc sapphire">&mdash;</td>
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
