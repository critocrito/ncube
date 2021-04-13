import c from "clsx";
import React from "react";

interface WorkspaceTagProps {
  kind?: "local" | "remote";
  className?: string;
}

const WorkspaceTag = ({kind = "local", className}: WorkspaceTagProps) => {
  const label = kind === "local" ? "local" : "remote";
  const classes = c(
    "tag flex flex-column justify-around sapphire",
    // This works since we only have two types of workspaces.
    kind === "local" ? "bg-solitude" : "bg-gray-25",
    className,
  );

  return (
    <div className={classes}>
      <span>{label}</span>
    </div>
  );
};

export default WorkspaceTag;
