/* eslint react/no-array-index-key: off */
import React from "react";

import {DataStats, SourceStats} from "../types";
import {capitalize} from "../utils";

interface StatsTableProps {
  stats: DataStats | SourceStats;
}

const StatsTable = ({stats}: StatsTableProps) => {
  // Tunr the variable length stats keys into a fixed length array. We do this in order to have a fixed number of columns.
  const keys = Object.keys(stats).reduce((memo, key, index) => {
    // eslint-disable-next-line no-param-reassign
    memo[index] = key;
    return memo;
  }, new Array(3).fill(undefined));

  return (
    <table className="w-100 collapse bn ml3 mr3 no-hover">
      <colgroup>
        <col className="w-third" />
        <col className="w-third" />
        <col className="w-third" />
      </colgroup>

      <thead>
        <tr>
          {keys.map((key, index) =>
            key === undefined ? (
              <th
                key={`header-${index}`}
                className="bn"
                aria-label="Empty header cell."
              />
            ) : (
              <th
                key={`${key}-header`}
                className="ba b--fair-pink tc b sapphire"
              >
                {capitalize(key)}
              </th>
            ),
          )}
        </tr>
      </thead>

      <tbody>
        <tr>
          {keys.map((key: keyof typeof stats | undefined, index) =>
            key === undefined ? (
              <td key={`row-${index}`} className="bn" />
            ) : (
              <td key={`${key}-row`} className="ba b--fair-pink tc sapphire">
                {stats[key]}
              </td>
            ),
          )}
        </tr>
      </tbody>
    </table>
  );
};

export default StatsTable;
