import React from "react";

import CreateSource from "../forms/create-source";
import {SourceReq, Workspace} from "../types";

interface SourceProps {
  onNext: (s: SourceReq) => void;
  onCancel: () => void;
  sourceReq: SourceReq;
  workspace: Workspace;
}

const Source = ({sourceReq, workspace, onNext, onCancel}: SourceProps) => {
  return (
    <CreateSource
      workspace={workspace}
      initialValues={sourceReq}
      onSubmit={onNext}
      onCancel={onCancel}
    />
  );
};

export default Source;
