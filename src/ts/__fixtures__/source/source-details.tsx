import React from "react";

import SourceDetails from "../../source/details";
import source from "../source.json";

export default (
  <div className="noto lh-copy pa2 flex flex-column bg-canvas">
    <SourceDetails onDelete={() => {}} source={source} />
  </div>
);
