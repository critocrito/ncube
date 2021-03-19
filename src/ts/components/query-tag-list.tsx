import React from "react";

import {SourceTag} from "../types";
import QueryTag from "./query-tag";

interface QueryTagListProps {
  tags: SourceTag[];
}

const QueryTagList = ({tags}: QueryTagListProps) => {
  return (
    <ul className="list pl0">
      {tags.map(({label, description}) => (
        <li key={label} className="mt2 mb2">
          <div className="flex">
            <QueryTag label={label} />
            <span className="ml2 solitude text-medium">{description}</span>
          </div>
        </li>
      ))}
    </ul>
  );
};

export default QueryTagList;
