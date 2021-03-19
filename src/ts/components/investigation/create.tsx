import React from "react";

import CreateInvestigationForm, {
  CreateInvestigationFormValues,
} from "../../forms/create-investigation";
import {Workspace} from "../../types";
import FormHandler from "../form-handler";

interface InvestigationCreateProps {
  workspace: Workspace;
  onCreate: (values: CreateInvestigationFormValues) => Promise<void>;
  onDone: () => void;
}

const InvestigationCreate = ({
  workspace,
  onCreate,
  onDone,
}: InvestigationCreateProps) => {
  return (
    <div className="flex flex-column">
      <h3>Add a new investigation to your workspace.</h3>

      <FormHandler
        onSave={onCreate}
        onDone={onDone}
        Form={CreateInvestigationForm}
        workspace={workspace}
      />
    </div>
  );
};

export default InvestigationCreate;
