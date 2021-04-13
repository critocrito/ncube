import c from "clsx";
import React from "react";

import {Platform} from "../types";
import SourceTag from "./source-tag";

interface LabeledSourceTagProps {
  label: string;
  className?: string;
}

const LabeledSourceTag = ({label, className}: LabeledSourceTagProps) => {
  let platform: Platform;

  switch (true) {
    case label.startsWith("youtube"): {
      platform = "youtube";
      break;
    }

    case label.startsWith("twitter"): {
      platform = "twitter";
      break;
    }

    default:
      platform = "http";
  }

  return (
    <div className={c("flex items-center", className)}>
      <SourceTag kind={platform} />
      <span className="ml3">{label}</span>
    </div>
  );
};

export default LabeledSourceTag;
