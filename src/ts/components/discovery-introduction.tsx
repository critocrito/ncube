import React from "react";

import {SourceReq} from "../types";
import Button from "./button";
import Description from "./description";
import LabeledSourceTag from "./labeled-source-tag";

interface IntroductionProps {
  onNext: () => void;
  sourceReq: SourceReq;
}

const Introduction = ({onNext, sourceReq}: IntroductionProps) => {
  const items = [{label: "Term", value: sourceReq.term}];

  return (
    <div className="flex flex-col">
      <Description items={items} />

      <LabeledSourceTag
        label={sourceReq.type}
        className="border border-solitude p-3"
      />

      <Button className="mt-3 ml-auto" size="large" onClick={onNext}>
        Preserve
      </Button>
    </div>
  );
};

export default Introduction;
