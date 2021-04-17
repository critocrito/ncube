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
            <h4 className="header4">{name}</h4>
            <p className="mt-2 mb-4">{description}</p>

            <div className="py-5">
              <ProcessForm
                template={template}
                values={value || {}}
                onSubmit={(v: Record<string, string>) => {
                  onDone({key, value: v});
                }}
                onCancel={onCancel}
              />
            </div>
          </div>
        );
      })}
    </>
  );
};

export default ProcessConfig;
