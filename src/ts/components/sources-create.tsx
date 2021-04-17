import React from "react";

import CreateSourceForm from "../forms/create-source";
import {saveSource} from "../lib/handlers";
import {Workspace} from "../types";
import FormHandler from "./form-handler";

interface SourcesCreateProps {
  workspace: Workspace;
  onDone: () => void;
}

const SourcesCreate = ({workspace, onDone}: SourcesCreateProps) => {
  return (
    <div className="flex flex-col">
      <h4 className="header4 mb-3">
        Add a new data source for your workspace.
      </h4>

      <FormHandler
        onSave={(values) => saveSource(workspace.slug, values)}
        onDone={onDone}
        Form={CreateSourceForm}
        workspace={workspace}
      />
    </div>
  );
};

export default SourcesCreate;
