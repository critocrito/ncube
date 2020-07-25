import React from "react";

import SegmentList from "../../workspace/segment-list";
import segments from "../segments.json";

const Wrapper = () => {
  return <SegmentList onExplore={() => {}} segments={segments} />;
};

export default (
  <div className="noto lh-copy pa2 flex flex-column bg-canvas">
    <Wrapper />
  </div>
);
