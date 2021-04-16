import React from "react";

import SendToVerificationForm, {
  SendToVerificationFormValues,
} from "../forms/send-to-verification";
import {Workspace} from "../types";
import FormHandler from "./form-handler";

interface DataSegmentsVerifyProps {
  workspace: Workspace;
  onCreate: (values: SendToVerificationFormValues) => Promise<void>;
  onDone: () => void;
}

const DataSegmentsVerify = ({
  workspace,
  onCreate,
  onDone,
}: DataSegmentsVerifyProps) => {
  return (
    <div className="flex flex-col">
      <p>Send a segment to an investigation for verification.</p>

      <FormHandler
        onSave={onCreate}
        onDone={onDone}
        Form={SendToVerificationForm}
        workspace={workspace}
      />
    </div>
  );
};

export default DataSegmentsVerify;
