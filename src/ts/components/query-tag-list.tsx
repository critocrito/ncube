import React from "react";

import {SourceTag} from "../types";
import QueryTag from "./query-tag";

interface QueryTagListProps {
  tags: SourceTag[];
}

const QueryTagList = ({tags}: QueryTagListProps) => {
  return (
    <ul>
      {tags.map(({label, description}) => (
        <li key={label} className="my-3">
          <div className="flex">
            <QueryTag label={label} />
            <span className="ml-3">{description}</span>
          </div>
        </li>
      ))}
    </ul>
  );
};

export default QueryTagList;
