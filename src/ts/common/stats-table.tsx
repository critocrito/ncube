/* eslint react/no-array-index-key: off */
import React from "react";

import {Stats} from "../types";

interface StatsTableProps {
  stats: Stats;
  statNames?: Array<{name: string; key: string} | undefined>;
}

const StatsTable = ({stats, statNames = []}: StatsTableProps) => {
  if (statNames.length === 0) return <div />;

  return (
    <table className="w-100 collapse bn ml3 mr3 no-hover">
      <colgroup>
        <col className="w-third" />
        <col className="w-third" />
        <col className="w-third" />
      </colgroup>

      <thead>
        <tr>
          {statNames.map((elem, index) =>
            elem === undefined ? (
              <th
                key={`header-${index}`}
                className="bn"
                aria-label="Empty header cell."
              />
            ) : (
              <th
                key={`${elem.key}-header-${index}`}
                className="ba b--fair-pink tc b sapphire"
              >
                {elem.name}
              </th>
            ),
          )}
        </tr>
      </thead>

      <tbody>
        <tr>
          {statNames.map((elem, index) =>
            elem === undefined ? (
              <td key={`row-${index}`} className="bn" />
            ) : (
              <td
                key={`${elem.key}-row-${index}`}
                className="ba b--fair-pink tc sapphire"
              >
                {stats[elem.key]}
              </td>
            ),
          )}
        </tr>
      </tbody>
    </table>
  );
};

export default StatsTable;
