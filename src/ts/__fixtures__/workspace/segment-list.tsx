import React from "react";

import {Workspace} from "../../types";
import SegmentList from "../../workspace/segment-list";
import workspace from "../local-workspace.json";
import segments from "../segments.json";

const Wrapper = () => {
  return (
    <SegmentList
      onExplore={() => {}}
      segments={segments}
      workspace={(workspace as unknown) as Workspace}
    />
  );
};

export default (
  <div className="noto lh-copy pa2 flex flex-column bg-canvas">
    <Wrapper />
  </div>
);
