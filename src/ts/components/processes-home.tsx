import React from "react";

import {voidFn} from "../lib/utils";
import {Process} from "../types";
import ProcessesTable from "./processes-table";

interface ProcessesHomeProps {
  processes: Process[];
  onShow?: (p: Process) => void;
  onRun?: (p: Process) => void;
}

const ProcessesHome = ({
  processes,
  onShow = voidFn,
  onRun = voidFn,
}: ProcessesHomeProps) => {
  return <ProcessesTable processes={processes} onShow={onShow} onRun={onRun} />;
};

export default ProcessesHome;
