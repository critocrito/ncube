import c from "classnames";
import React from "react";

interface SourceTagProps {
  kind?: "youtube" | "twitter" | "http";
  className?: string;
}

const SourceTag = ({kind = "http", className}: SourceTagProps) => {
  let label;
  let classes;
  const baseClasses = "short-tag flex flex-column justify-around";

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

  return (
    <div className={classes}>
      <span>{label}</span>
    </div>
  );
};

export default SourceTag;
