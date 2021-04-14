import c from "clsx";
import React from "react";

interface WorkspaceTagProps {
  kind?: "local" | "remote";
}

const WorkspaceTag = ({kind = "local"}: WorkspaceTagProps) => {
  const label = kind === "local" ? "local" : "remote";
  const classes = c(
    "text-sapphire text-sm font-bold uppercase",
    "items-center text-center px-3 py-0.5 w-20 rounded-full",
    {
      "bg-solitude": kind === "local",
      "bg-gray-light": kind === "remote",
    },
  );

  return <span className={classes}>{label}</span>;
};

export default WorkspaceTag;
