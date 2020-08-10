import React from "react";

import Button from "../common/button";
import LabeledSourceTag from "../common/labeled-source-tag";
import {SourceReq} from "../types";

interface IntroductionProps {
  onNext: () => void;
  sourceReq: SourceReq;
}

const Introduction = ({onNext, sourceReq}: IntroductionProps) => {
  return (
    <div className="flex flex-column">
      <p className="mb2">Type of source detected</p>
      <LabeledSourceTag label={sourceReq.type} className="ba b--solitude pa3" />

      <p className="mb1">Term</p>
      <div className="underline">{sourceReq.term}</div>

      <Button className="mt4" size="large" onClick={onNext}>
        Preserve
      </Button>
    </div>
  );
};

export default Introduction;
