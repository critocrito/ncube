import React from "react";

import SourceDetails from "../../components/sources-details";
import {source} from "../data";

export default (
  <div className="noto lh-copy pa2 flex flex-column bg-canvas">
    <SourceDetails onDelete={() => {}} source={source} />
  </div>
);
