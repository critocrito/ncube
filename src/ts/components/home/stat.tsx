import React from "react";

import dataIcon from "../../../../resources/public/images/icon_data.svg";
import investigationIcon from "../../../../resources/public/images/icon_investigation.svg";
import processIcon from "../../../../resources/public/images/icon_process.svg";
import queryIcon from "../../../../resources/public/images/icon_query.svg";

interface StatProps {
  kind: "query" | "data" | "process" | "investigation";
  value: number;
}

const Stat = ({kind, value}: StatProps) => {
  let icon;
  let label;
  if (kind === "query") {
    label = "Queries";
    icon = queryIcon;
  } else if (kind === "data") {
    label = "Data";
    icon = dataIcon;
  } else if (kind === "process") {
    label = "Process";
    icon = processIcon;
  } else if (kind === "investigation") {
    label = "Investigation";
    icon = investigationIcon;
  }

  return (
    <div className="ml2 mr2 flex items-center">
      <img src={icon} className="mr1" alt={`Icon for the ${kind} stat.`} />
      <div className="b text-medium ttu sapphire">{`${label}: ${value}`}</div>
    </div>
  );
};

export default Stat;
