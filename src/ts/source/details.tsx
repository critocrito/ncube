import React from "react";

import Button from "../common/button";
import CopyAndPaste from "../common/copy-and-paste";
import QueryTag from "../common/query-tag";
import SourceTag from "../common/source-tag";
import {Source} from "../types";

interface SourceDetailsProps {
  source: Source;
  onDelete: () => void;
}

const SourceDetails = ({source, onDelete}: SourceDetailsProps) => {
  let platform: "youtube" | "twitter" | "http";

  switch (true) {
    case source.type.startsWith("youtube"): {
      platform = "youtube";
      break;
    }

    case source.type.startsWith("twitter"): {
      platform = "twitter";
      break;
    }

    default:
      platform = "http";
  }

  return (
    <div className="flex flex-column">
      <h3 className="header3">Source Details</h3>

      <p className="mb2">Term</p>
      <div className="flex items-start justify-between">
        <span className="w-90">{source.term}</span>
        <CopyAndPaste value={source.term} />
      </div>

      <p className="mb2">Type</p>
      <div className="flex items-center">
        <SourceTag kind={platform} />
        <span className="ml3">{source.type}</span>
      </div>

      {source.tags.length > 0 && (
        <div>
          <p className="mb1">Tags</p>
          <ul className="list pl0">
            {source.tags.map(({label}) => (
              <li key={label} className="mt2 mb2">
                <QueryTag label={label} />
              </li>
            ))}
          </ul>
        </div>
      )}

      <Button className="mt4 ml-auto" size="large" onClick={onDelete}>
        Delete Source
      </Button>
    </div>
  );
};

export default SourceDetails;
