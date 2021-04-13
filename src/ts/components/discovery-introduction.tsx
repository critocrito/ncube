import React from "react";

import {SourceReq} from "../types";
import Button from "./button";
import LabeledSourceTag from "./labeled-source-tag";

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
