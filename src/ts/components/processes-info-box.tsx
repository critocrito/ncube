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
    <div className="text-md flex items-center">
      <img src={successIcon} alt="Process is configured." className="h1 w1" />
      <span className="ml2 tl">
        All requirements for this process are fulfilled.
      </span>
    </div>
  ) : (
    <div className="flex items-center justify-between hover">
      <div className="text-md flex items-center">
        <img
          src={failIcon}
          alt="Process requires configuration."
          className="h1 w1"
        />
        <span className="ml2 tl">This process needs to be configured.</span>
      </div>
      <Button onClick={onSetup} kind="secondary" size="small">
        Set Up
      </Button>
    </div>
  );
};

export default ProcessesInfoBox;
