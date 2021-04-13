import React from "react";

import ProcessConfig from "../../components/processes-config";
import {process1 as process} from "../data";

export default (
  <div className="noto lh-copy pa2 flex flex-column bg-canvas">
    <ProcessConfig onCancel={() => {}} onDone={() => {}} process={process} />
  </div>
);
