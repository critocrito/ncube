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
    <div className="flex flex-column">
      <h3 className="sapphire">Add a new data source for your workspace.</h3>

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
