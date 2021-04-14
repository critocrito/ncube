import React, {useEffect, useState} from "react";

import dataIcon from "../../../resources/public/images/icon_data.svg";
import investigationIcon from "../../../resources/public/images/icon_investigation.svg";
import processIcon from "../../../resources/public/images/icon_process.svg";
import sourceIcon from "../../../resources/public/images/icon_source.svg";
import LoadingSpinner from "./loading-spinner";

interface StatProps {
  kind: "source" | "data" | "process" | "investigation";
  fetchStat: () => Promise<number>;
}

const Stat = ({kind, fetchStat}: StatProps) => {
  const [fetchDone, setFetchDone] = useState(false);
  const [statValue, setStatValue] = useState(0);

  useEffect(() => {
    const f = async () => {
      const stat = await fetchStat();
      setStatValue(stat);
      setFetchDone(true);
    };
    f();
  }, [fetchStat]);

  let icon;
  let label;
  if (kind === "source") {
    label = "Sources";
    icon = sourceIcon;
  } else if (kind === "data") {
    label = "Data";
    icon = dataIcon;
  } else if (kind === "process") {
    label = "Processes";
    icon = processIcon;
  } else if (kind === "investigation") {
    label = "Investigations";
    icon = investigationIcon;
  }

  if (!fetchDone) return <LoadingSpinner />;

  return (
    <div className="ml2 mr2 flex items-center">
      <img src={icon} className="mr1" alt={`Icon for the ${kind} stat.`} />
      <div className="b text-md ttu text-sapphire">
        {label}: {statValue === 0 ? <span>&mdash;</span> : statValue}
      </div>
    </div>
  );
};

export default Stat;
