import React from "react";

import WorkspaceTag from "../../components/workspace-tag";

export default (
  <div className="noto lh-copy pa2 flex flex-column">
    <WorkspaceTag />
    <WorkspaceTag kind="remote" />
  </div>
);
