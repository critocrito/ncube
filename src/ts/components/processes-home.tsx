import React from "react";

import {voidFn} from "../lib/utils";
import {Process} from "../types";
import ProcessesCard from "./processes-card";

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
  return (
    <div className="flex flex-col space-y-8">
      {processes.map((process) => (
        <ProcessesCard
          key={process.id}
          process={process}
          onShow={() => onShow(process)}
          onRun={() => onRun(process)}
        />
      ))}
    </div>
  );
};

export default ProcessesHome;
