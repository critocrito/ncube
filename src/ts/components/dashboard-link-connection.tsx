import React from "react";

import {connectionDetailsUpload} from "../lib/validations";
import {ConnectionDetails} from "../types";
import Button from "./button";
import FileUpload from "./file-upload";

interface DashboardLinkConnectionProps {
  onSubmit: (details: ConnectionDetails) => void;
  onError: (error: string) => void;
  onCancel: () => void;
}

const DashboardLinkConnection = ({
  onSubmit,
  onError,
  onCancel,
}: DashboardLinkConnectionProps) => {
  return (
    <div>
      <FileUpload
        handleUpload={(details) => {
          try {
            // FIXME: Can the validation return the proper type so I don't have to convert?
            connectionDetailsUpload.validateSync(details);
            onSubmit((details as unknown) as ConnectionDetails);
          } catch {
            onError("The connection details format is invalid.");
          }
        }}
        handleError={onError}
      />
      <div className="flex justify-between ml-auto fr mt2">
        <Button kind="secondary" size="large" onClick={onCancel}>
          Cancel
        </Button>
      </div>
    </div>
  );
};

export default DashboardLinkConnection;
