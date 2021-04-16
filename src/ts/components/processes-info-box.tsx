import React from "react";

import failIcon from "../../../resources/public/images/icon_fail.svg";
import successIcon from "../../../resources/public/images/icon_success.svg";
import Button from "./button";

interface ProcessesInfoBoxProps {
  isSetup: boolean;
  onSetup: () => void;
}

const ProcessesInfoBox = ({isSetup, onSetup}: ProcessesInfoBoxProps) => {
  return isSetup ? (
    <div className="flex items-start">
      <img src={successIcon} alt="Process is configured." className="h-5 w-5" />
      <span className="ml-3 leading-tight">
        All requirements for this process are fulfilled.
      </span>
    </div>
  ) : (
    <div className="flex items-start">
      <div className="flex items-start">
        <img
          src={failIcon}
          alt="Process requires configuration."
          className="h-5 w-5"
        />
        <span className="ml-3 leading-tight">
          This process needs to be configured.
        </span>
      </div>

      <Button onClick={onSetup} kind="secondary" size="small">
        Set Up
      </Button>
    </div>
  );
};

export default ProcessesInfoBox;
