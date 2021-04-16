import React from "react";

import ProcessForm from "../forms/configure-process";
import {Process, ProcessConfigReq} from "../types";

interface ProcessConfigProps {
  process: Process;
  onDone: (request: ProcessConfigReq) => void;
  onCancel: () => void;
}

const ProcessConfig = ({
  process: {config},
  onDone,
  onCancel,
}: ProcessConfigProps) => {
  return (
    <>
      {config.map(({key, name, description, template, value}) => {
        return (
          <div key={name}>
            <h4>{name}</h4>
            <p>{description}</p>

            <ProcessForm
              template={template}
              values={value || {}}
              onSubmit={(v: Record<string, string>) => {
                onDone({key, value: v});
              }}
              onCancel={onCancel}
            />
          </div>
        );
      })}
    </>
  );
};

export default ProcessConfig;
