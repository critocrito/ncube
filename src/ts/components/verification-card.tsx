import React from "react";

import {Platform, SegmentUnit} from "../types";
import SourceTag from "./source-tag";

interface VerificationCardProps {
  unit: SegmentUnit;
}

const VerificationCard = ({unit}: VerificationCardProps) => {
  let platform: Platform;

  switch (true) {
    case unit.source.startsWith("youtube"): {
      platform = "youtube";
      break;
    }

    case unit.source.startsWith("twitter"): {
      platform = "twitter";
      break;
    }

    default:
      platform = "http";
  }

  return (
    <div className="ba b--solitude ma2 flex flex-column justify-between bg-white">
      <div className="pa2">{unit.title}</div>

      <div className="flex items-center justify-between bg-canvas h2 pa2">
        <SourceTag kind={platform} className="br b--solitude mv2" />
        <div>&nbsp;</div>
      </div>
    </div>
  );
};

export default VerificationCard;
