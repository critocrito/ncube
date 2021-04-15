import c from "clsx";
import React from "react";

import {Platform} from "../types";

interface SourceTagProps {
  kind?: Platform;
  className?: string;
}

const SourceTag = ({kind = "http", className}: SourceTagProps) => {
  let label;
  let classes;
  const baseClasses =
    "w-14 text-sm text-white text-center px-3 py-0.5 rounded-full";

  switch (kind) {
    case "youtube": {
      label = "YT";
      classes = c(baseClasses, "bg-youtube", className);
      break;
    }
    case "twitter": {
      label = "TW";
      classes = c(baseClasses, "bg-twitter", className);
      break;
    }
    default: {
      label = "URL";
      classes = c(baseClasses, "bg-http", className);
      break;
    }
  }

  return <span className={classes}>{label}</span>;
};

export default SourceTag;
