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
    <div className="border border-solitude flex flex-col justify-between bg-white">
      <div className="p-2">{unit.title}</div>

      <div className="flex items-center justify-between bg-canvas h-12 p-1.5">
        <SourceTag kind={platform} className="border-r border-solitude my-3" />
        <div>&nbsp;</div>
      </div>
    </div>
  );
};

export default VerificationCard;
