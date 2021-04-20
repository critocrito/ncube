import React from "react";

import dataIcon from "../svg/data.svg";
import investigationIcon from "../svg/investigation.svg";
import processIcon from "../svg/process.svg";
import sourceIcon from "../svg/source.svg";
import {unreachable} from "../lib/utils";

interface PanelSidebarMenuItemProps {
  kind: "source" | "data" | "process" | "investigation";
  onClick: () => void;
}

const PanelSidebarMenuItem = ({kind, onClick}: PanelSidebarMenuItemProps) => {
  let icon;
  let label;
  let desc;

  switch (kind) {
    case "source": {
      icon = sourceIcon;
      label = "Sources";
      desc = "Go to workspace sources.";
      break;
    }
    case "data": {
      icon = dataIcon;
      label = "Data";
      desc = "Go to workspace data.";
      break;
    }
    case "process": {
      icon = processIcon;
      label = "Processes";
      desc = "Go to workspace processes.";
      break;
    }
    case "investigation": {
      icon = investigationIcon;
      label = "Investigations";
      desc = "Go to workspace investigations.";
      break;
    }
    default:
      return unreachable(
        `No kind for ${kind} found for this sidebar menu item.`,
      );
  }

  return (
    <button className="flex items-center dim" onClick={onClick}>
      <img className="w-5 h-5" src={icon} alt={desc} />
      <span className="ml-3">{label}</span>
    </button>
  );
};

export default PanelSidebarMenuItem;
