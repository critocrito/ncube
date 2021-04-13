import React from "react";

import CreateWorkspaceForm from "../forms/create-workspace";
import {saveWorkspace} from "../lib/handlers";
import FormHandler from "./form-handler";

interface DashboardCreateWorkspaceProps {
  onDone: () => void;
}

const DashboardCreateWorkspace = ({onDone}: DashboardCreateWorkspaceProps) => {
  return (
    <FormHandler
      onSave={saveWorkspace}
      onDone={onDone}
      Form={CreateWorkspaceForm}
    />
  );
};

export default DashboardCreateWorkspace;
