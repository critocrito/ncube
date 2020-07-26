import React from "react";

import Button from "../common/button";
import SourceTag from "../common/source-tag";
import {SourceReq} from "../types";

interface IntroductionProps {
  onNext: () => void;
  sourceReq: SourceReq;
}

const Introduction = ({onNext, sourceReq}: IntroductionProps) => {
  let platform: "youtube" | "twitter" | "http";

  switch (true) {
    case sourceReq.type.startsWith("youtube"): {
      platform = "youtube";
      break;
    }

    case sourceReq.type.startsWith("twitter"): {
      platform = "twitter";
      break;
    }

    default:
      platform = "http";
  }

  return (
    <div className="flex flex-column">
      <p>Type of source detected</p>
      <div className="ba b--solitude pa3 flex items-center">
        <SourceTag kind={platform} />
        <span className="ml3">{sourceReq.type}</span>
      </div>

      <p>Term</p>
      <div>{sourceReq.term}</div>

      <Button className="mt4" size="large" onClick={onNext}>
        Preserve
      </Button>
    </div>
  );
};

export default Introduction;
