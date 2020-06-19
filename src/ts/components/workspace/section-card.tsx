import React from "react";

import dataIcon from "../../../../resources/public/images/icon_data.svg";
import investigationIcon from "../../../../resources/public/images/icon_investigation.svg";
import processIcon from "../../../../resources/public/images/icon_process.svg";
import sourceIcon from "../../../../resources/public/images/icon_source.svg";
import Button from "../base/button";

interface SectionCardProps {
  kind: "source" | "data" | "process" | "investigation";
}

const dummy =
  "I'm some sort of description. What I will be, I don't know yet. But I'm convinced, it will be mganificient.";

const SectionCard = ({kind}: SectionCardProps) => {
  let title;
  let icon;
  let label;

  if (kind === "source") {
    title = "Sources";
    icon = sourceIcon;
    label = "Manage";
  } else if (kind === "data") {
    title = "Data";
    icon = dataIcon;
    label = "Explore";
  } else if (kind === "process") {
    title = "Processes";
    icon = processIcon;
    label = "Set Up";
  } else if (kind === "investigation") {
    title = "Investigations";
    icon = investigationIcon;
    label = "Verify";
  }

  return (
    <div className="h4 bg-white shadow-1 flex items-center justify-between mb4">
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
          <p className="text-small">{dummy}</p>
        </div>
      </div>
      <div className="pr2">
        <Button onClick={() => {}}>{label}</Button>
      </div>
    </div>
  );
};

export default SectionCard;
