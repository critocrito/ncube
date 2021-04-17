import React from "react";

import CreateInvestigationForm, {
  CreateInvestigationFormValues,
} from "../forms/create-investigation";
import {Workspace} from "../types";
import FormHandler from "./form-handler";

interface InvestigationsCreateProps {
  workspace: Workspace;
  onCreate: (values: CreateInvestigationFormValues) => Promise<void>;
  onDone: () => void;
}

const InvestigationsCreate = ({
  workspace,
  onCreate,
  onDone,
}: InvestigationsCreateProps) => {
  return (
    <div className="flex flex-col">
      <h4 className="header4 mb-3">
        Add a new investigation to your workspace.
      </h4>

      <FormHandler
        onSave={onCreate}
        onDone={onDone}
        Form={CreateInvestigationForm}
        workspace={workspace}
      />
    </div>
  );
};

export default InvestigationsCreate;
