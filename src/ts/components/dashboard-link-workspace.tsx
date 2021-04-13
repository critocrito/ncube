import React from "react";

import LinkWorkspaceForm, {
  LinkWorkspaceFormValues,
} from "../forms/link-workspace";
import {saveWorkspace} from "../lib/handlers";
import {ConnectionDetails} from "../types";
import FormHandler from "./form-handler";

interface DashboardLinkWorkspaceProps {
  connection: ConnectionDetails;
  onDone: () => void;
}

const DashboardLinkWorkspace = ({
  connection,
  onDone,
}: DashboardLinkWorkspaceProps) => {
  return (
    <FormHandler
      onSave={saveWorkspace}
      onDone={onDone}
      Form={LinkWorkspaceForm}
      initialValues={
        (connection as unknown) as Partial<LinkWorkspaceFormValues>
      }
    />
  );
};

export default DashboardLinkWorkspace;
