/* eslint react/no-array-index-key: off */
import React from "react";

import {capitalize} from "../lib/utils";
import {DataStats, SourceStats} from "../types";

interface StatsTableProps {
  stats: DataStats | SourceStats;
}

const StatsTable = ({stats}: StatsTableProps) => {
  // Turn the variable length stats keys into a fixed length array. We do this
  // to end up with a fixed number of columns.
  const keys = Object.keys(stats).reduce((memo, key, index) => {
    // eslint-disable-next-line no-param-reassign
    memo[index] = key;
    return memo;
  }, Array.from({length: 3}).fill(undefined) as string[]);

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
                className="ba b--fair-pink tc b text-sapphire"
              >
                {capitalize(key)}
              </th>
            ),
          )}
        </tr>
      </thead>

      <tbody>
        <tr>
          {((keys as unknown) as (keyof typeof stats)[]).map((key, index) =>
            stats[key] === undefined ? (
              <td key={`row-${index}`} className="bn" />
            ) : (
              <td
                key={`${key}-row`}
                className="ba b--fair-pink tc text-sapphire"
              >
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
