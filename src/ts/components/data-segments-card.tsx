import React from "react";

import {statSegmentsUnits} from "../lib/http";
import {Segment, Workspace} from "../types";
import Button from "./button";
import Card from "./card";
import DataSegmentsActions from "./data-segments-actions";
import Stat from "./stat";

interface DataSegmentsCardProps {
  workspace: Workspace;
  segment: Segment;
  onShow: () => void;
  onVerify: () => void;
  onDelete: () => void;
}

const DataSegmentsCard = ({
  workspace,
  segment: {query, title, slug},
  onShow,
  onVerify,
  onDelete,
}: DataSegmentsCardProps) => {
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
            <th className="border-l border-r border-t border-fair-pink font-bold text-sapphire p-2">
              <h4 className="header4 text-left">{title}</h4>
            </th>
            <th className="border border-fair-pink text-center text-sapphire">
              Query
            </th>
            <th className="border border-fair-pink text-center text-sapphire">
              Units
            </th>
            <th className="border border-fair-pink text-center text-sapphire">
              New
            </th>
          </tr>
        </thead>

        <tbody>
          <tr>
            <td className="border-l border-r border-fair-pink text-sapphire py-4">
              <Button kind="secondary" size="small" onClick={onShow}>
                Explore
              </Button>
            </td>
            <td className="border border-fair-pink text-center text-sapphire">
              {query}
            </td>
            <td className="border border-fair-pink text-center text-sapphire">
              <Stat fetchStat={() => statSegmentsUnits(workspace.slug, slug)} />
            </td>
            <td className="border border-fair-pink text-center text-sapphire">
              &mdash;
            </td>
          </tr>
        </tbody>
      </table>

      <div className="h-full ml-auto">
        <DataSegmentsActions onVerify={onVerify} onDelete={onDelete} />
      </div>
    </Card>
  );
};

export default DataSegmentsCard;
