import React from "react";

import Button from "../common/button";
import {SourceReq} from "../types";

interface IntroductionProps {
  onNext: () => void;
  sourceReq: SourceReq;
}

const Introduction = ({onNext, sourceReq}: IntroductionProps) => {
  return (
    <div>
      <p>{sourceReq.type}</p>
      <p>{sourceReq.term}</p>
      <div className="flex justify-around">
        <Button size="large" onClick={onNext}>
          Preserve
        </Button>
      </div>
    </div>
  );
};

export default Introduction;
