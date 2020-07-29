import React from "react";

import {Process} from "../types";
import ProcessCard from "./process-card";

interface ProcessListProps {
  processes: Process[];
  onClick: (p: Process) => void;
  onRun: (p: Process) => void;
}

const ProcessList = ({processes, onClick, onRun}: ProcessListProps) => {
  return (
    <div className="flex flex-column">
      {processes.map((process) => (
        <ProcessCard
          key={process.id}
          process={process}
          onClick={() => onClick(process)}
          onRun={() => onRun(process)}
        />
      ))}
    </div>
  );
};

export default ProcessList;
