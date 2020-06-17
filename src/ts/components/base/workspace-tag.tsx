import c from "classnames";
import React from "react";

interface WorkspaceTagProps {
  kind?: "local" | "remote";
}

const WorkspaceTag = ({kind = "local"}: WorkspaceTagProps) => {
  const label = kind === "local" ? "local" : "remote";
  const classes = c(
    "tag flex flex-column justify-around sapphire",
    // This works since we only have two types of workspaces.
    kind === "local" ? "bg-solitude" : "bg-gray-25",
  );

  return (
    <div className={classes}>
      <span>{label}</span>
    </div>
  );
};

export default WorkspaceTag;
