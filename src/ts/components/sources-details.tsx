import React from "react";

import {Source} from "../types";
import Button from "./button";
import CopyAndPaste from "./copy-and-paste";
import QueryTagList from "./query-tag-list";
import SourceTag from "./source-tag";
import Description from "./description";
import LabeledSourceTag from "./labeled-source-tag";

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

  const items = [
    {
      label: "Term",
      value: (
        <div className="flex items-center space-x-5">
          <span>{source.term}</span>
          <CopyAndPaste value={source.term} />
        </div>
      ),
    },
    {
      label: "Type",
      value: <LabeledSourceTag label={platform} />,
    },
    {
      label: "Tags",
      value: <QueryTagList tags={source.tags} />,
    },
  ];
  return (
    <div className="flex flex-col w-full">
      <h4 className="header4">Source Details</h4>

      <Description items={items} />

      <div className="flex mt-3 ml-auto">
        <Button size="large" onClick={onDelete}>
          Delete Source
        </Button>
      </div>
    </div>
  );
};

export default SourceDetails;
