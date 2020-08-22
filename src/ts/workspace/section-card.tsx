import React from "react";

import dataIcon from "../../../resources/public/images/icon_data.svg";
import investigationIcon from "../../../resources/public/images/icon_investigation.svg";
import processIcon from "../../../resources/public/images/icon_process.svg";
import sourceIcon from "../../../resources/public/images/icon_source.svg";
import Button from "../common/button";
import StatsTable from "../common/stats-table";
import {DataStats, SourceStats} from "../types";

interface SectionCardProps {
  kind: "source" | "data" | "process" | "investigation";
  onClick: () => void;
  stats?: DataStats | SourceStats;
}

const dummy =
  "I'm some sort of description. What I will be, I don't know yet. But I'm convinced, it will be mganificient.";

const SectionCard = ({kind, stats, onClick = () => {}}: SectionCardProps) => {
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
    <div className="h4 bg-white pa3 shadow-4 flex items-center justify-between mb4">
      <div className="w-40 pa2">
        <div className="flex flex-column pl2">
          <div className="flex h3 items-center w-100">
            <img
              src={icon}
              className="h2 w2"
              alt={`Icon for the ${kind} section.`}
            />
            <h4 className="header4 pl2">{title}</h4>
          </div>
          <p className="text-small">{description}</p>
        </div>
      </div>

      <div className="w-40">{stats && <StatsTable stats={stats} />}</div>

      <div className="pr3 mt3 h-100 flex flex-column">
        <Button onClick={onClick}>{label}</Button>
      </div>
    </div>
  );
};

export default SectionCard;
