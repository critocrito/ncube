import React from "react";

import dataIcon from "../../../resources/public/images/icon_data.svg";
import investigationIcon from "../../../resources/public/images/icon_investigation.svg";
import processIcon from "../../../resources/public/images/icon_process.svg";
import sourceIcon from "../../../resources/public/images/icon_source.svg";
import {DataStats, SourceStats} from "../types";
import Button from "./button";
import StatsTable from "./stats-table";
import Card from "./card";

interface WorkspacesCardProps {
  kind: "source" | "data" | "process" | "investigation";
  onShow: () => void;
  stats?: DataStats | SourceStats;
}

const WorkspacesCard = ({kind, stats, onShow}: WorkspacesCardProps) => {
  let title;
  let icon;
  let label;
  let description;

  if (kind === "source") {
    title = "Sources";
    icon = sourceIcon;
    label = "Manage";
    description = "Sources are links to data that you would like to preserve.";
  } else if (kind === "data") {
    title = "Data";
    icon = dataIcon;
    label = "Explore";
    description = "A preview of the collected data from the processed sources.";
  } else if (kind === "process") {
    title = "Processes";
    icon = processIcon;
    label = "Set Up";
    description = "Processes responsible for fetching data from the sources.";
  } else if (kind === "investigation") {
    title = "Investigations";
    icon = investigationIcon;
    label = "Verify";
    description = "Defined methodologies to verify the collected data";
  }

  return (
    <Card>
      <div className="w-64 flex flex-col justify-between h-full">
        <div className="flex items-center pb-1.5">
          <img
            src={icon}
            className="h-8 w-8"
            alt={`Icon for the ${kind} section.`}
          />
          <h4 className="header4 pl-2">{title}</h4>
        </div>
        <p className="text-sm">{description}</p>
      </div>

      <div className="w-72 h-full ml-4">
        {stats && <StatsTable stats={stats} />}
      </div>

      <div className="pr-3 h-full ml-auto">
        <Button onClick={onShow}>{label}</Button>
      </div>
    </Card>
  );
};

export default WorkspacesCard;
