import React from "react";

import dataIcon from "../../../resources/public/images/icon_data.svg";
import {statDataSegments, statDataSources, statDataTotal} from "../lib/http";
import {Workspace} from "../types";
import Button from "./button";
import Stat from "./stat";

interface DataCardProps {
  workspace: Workspace;
  onShow: () => void;
}

const DataCard = ({workspace: {slug}, onShow}: DataCardProps) => {
  return (
    <div className="h4 bg-white pa3 shadow-4 flex justify-between mb4">
      <div className="w-80 flex items-center justify-between">
        <div className="w-40">
          <div className="flex flex-column pl2">
            <div className="flex h3 items-center w-100">
              <img
                src={dataIcon}
                className="h2 w2"
                alt="Icon for the data section."
              />
              <h4 className="header4 pl2">Data</h4>
            </div>
            <p className="text-small">&nbsp;</p>
          </div>
        </div>

        <div className="w-60 h-100 flex flex-column">
          <table className="w-100 h-100 collapse bn card">
            <colgroup>
              <col className="w-third" />
              <col className="w-third" />
              <col className="w-third" />
            </colgroup>

            <thead>
              <tr>
                <th className="ba b--fair-pink tc b sapphire nowrap">
                  Total Data
                </th>
                <th className="ba b--fair-pink tc b sapphire nowrap">
                  Total Sources
                </th>
                <th className="ba b--fair-pink tc b sapphire nowrap">
                  Total Segments
                </th>
              </tr>
            </thead>

            <tbody>
              <tr>
                <td className="ba b--fair-pink tc sapphire">
                  <Stat fetchStat={() => statDataTotal(slug)} />
                </td>
                <td className="ba b--fair-pink tc sapphire">
                  <Stat fetchStat={() => statDataSources(slug)} />
                </td>
                <td className="ba b--fair-pink tc sapphire">
                  <Stat fetchStat={() => statDataSegments(slug)} />
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>

      <div className="pr3 h-100 flex flex-column">
        <Button onClick={onShow}>Explore</Button>
      </div>
    </div>
  );
};

export default DataCard;
