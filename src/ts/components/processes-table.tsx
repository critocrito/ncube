import React from "react";

import {Process} from "../types";
import ProcessesCard from "./processes-card";

interface ProcessesTableProps {
  processes: Process[];
  onShow: (p: Process) => void;
  onRun: (p: Process) => void;
}

const ProcessesTable = ({processes, onShow, onRun}: ProcessesTableProps) => {
  return (
    <div className="flex flex-column">
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

export default ProcessesTable;
