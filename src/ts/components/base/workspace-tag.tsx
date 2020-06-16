import c from "classnames";
import React from "react";

interface WorkspaceTagProps {
  kind?: "local" | "remote";
}

const WorkspaceTag = ({kind = "local"}: WorkspaceTagProps) => {
  const label = kind === "local" ? "local" : "remote";
  const classes = c(
    "tag flex flex-column justify-around ma1 br4 back-to-reality text-middle tc b ttu",
    // This works since we only have two types of workspaces.
    kind === "local" ? "bg-local-workspace" : "bg-remote-workspace",
  );

  return (
    <div className={classes}>
      <span>{label}</span>
    </div>
  );
};

export default WorkspaceTag;
