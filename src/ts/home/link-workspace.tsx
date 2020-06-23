import {useMachine} from "@xstate/react";
import React from "react";

import Button from "../common/button";
import Error from "../common/error";
import Fatal from "../common/fatal";
import FileUpload from "../common/file-upload";
import FormHandler from "../common/form-handler";
import LinkWorkspaceForm from "../forms/link-workspace";
import {saveWorkspace} from "../handlers/workspace";
import BasicPanel from "../layout/basic-panel";
import machine from "../machines/link-workspace";
import {useServiceLogger} from "../utils";
import {connectionDetailsUpload} from "../validations";

interface LinkWorkspaceProps {
  onDone: () => void;
}

const LinkWorkspace = ({onDone}: LinkWorkspaceProps) => {
  const [state, send, service] = useMachine(machine, {
    actions: {
      done: (_ctx) => onDone(),
    },
  });

  useServiceLogger(service, machine.id);

  switch (true) {
    case state.matches("fileUpload"):
      return (
        <BasicPanel
          header="Upload your connection file."
          description="You should have received a connection from the operator of the remote Ncube installation."
        >
          <div>
            <FileUpload
              handleUpload={(details) => {
                try {
                  connectionDetailsUpload.validateSync(details);
                  send("NEXT", {details});
                } catch {
                  send("ERROR", {
                    error: "The connection details format is invalid.",
                  });
                }
              }}
              handleError={(error: string) => send("ERROR", {error})}
            />
            <div className="flex justify-between ml-auto fr mt2">
              <Button
                kind="secondary"
                size="large"
                onClick={() => send("CANCEL")}
              >
                Cancel
              </Button>
            </div>
          </div>
        </BasicPanel>
      );

    case state.matches("linkWorkspace"):
      return (
        <BasicPanel
          header="Fill in some details about your new workspace."
          description="Before you can create a new local workspace, please fill in some basic configuration."
        >
          <FormHandler
            onSave={saveWorkspace}
            onDone={() => send("DONE")}
            Form={LinkWorkspaceForm}
            initialValues={state.context.values}
          />
        </BasicPanel>
      );

    case state.matches("error"):
      return (
        <Error
          msg={state.context.error || "Failed to fetch workspaces."}
          recover={() => send("RETRY")}
        />
      );

    default:
      return (
        <Fatal
          msg={`${machine.id} machine didn't match any valid state: ${state.value}`}
        />
      );
  }
};

export default LinkWorkspace;
