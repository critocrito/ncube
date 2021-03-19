import {useMachine} from "@xstate/react";
import React from "react";

import LinkWorkspaceForm from "../../forms/link-workspace";
import {saveWorkspace} from "../../lib/handlers";
import {useServiceLogger} from "../../lib/utils";
import {connectionDetailsUpload} from "../../lib/validations";
import machine from "../../machines/link-workspace";
import Button from "../button";
import Error from "../error";
import FileUpload from "../file-upload";
import FormHandler from "../form-handler";
import Unreachable from "../unreachable";

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
      );

    case state.matches("linkWorkspace"):
      return (
        <FormHandler
          onSave={saveWorkspace}
          onDone={() => send("DONE")}
          Form={LinkWorkspaceForm}
          initialValues={state.context.values}
        />
      );

    case state.matches("error"):
      return (
        <Error
          msg={state.context.error || "Failed to fetch workspaces."}
          recover={() => send("RETRY")}
        />
      );

    default:
      return <Unreachable machine={machine.id} state={state.value} />;
  }
};

export default LinkWorkspace;
