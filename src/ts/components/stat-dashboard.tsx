import React, {useEffect, useState} from "react";

import dataIcon from "../../../resources/public/images/icon_data.svg";
import investigationIcon from "../../../resources/public/images/icon_investigation.svg";
import processIcon from "../../../resources/public/images/icon_process.svg";
import sourceIcon from "../../../resources/public/images/icon_source.svg";
import LoadingSpinner from "./loading-spinner";
import {useMobileSize} from "../lib/hooks";

interface StatProps {
  kind: "source" | "data" | "process" | "investigation";
  fetchStat: () => Promise<number>;
}

const Stat = ({kind, fetchStat}: StatProps) => {
  const [fetchDone, setFetchDone] = useState(false);
  const [statValue, setStatValue] = useState(0);
  const isMobile = useMobileSize();

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
    <div className="flex items-center">
      <img
        src={icon}
        className="h-5 w-5 mr-3"
        alt={`Icon for the ${kind} stat.`}
      />
      <div className="text-sm font-bold uppercase text-sapphire">
        {isMobile ? "" : `${label}: `}
        {statValue === 0 ? <span>&mdash;</span> : statValue}
      </div>
    </div>
  );
};

export default Stat;
