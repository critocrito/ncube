import React from "react";

import iconHelp from "../../../resources/public/images/icon_help.svg";

interface ButtonHelpProps {
  onClick: () => void;
  isDisabled?: boolean;
}

const ButtonHelp = ({onClick, isDisabled = false}: ButtonHelpProps) => {
  return (
    <button
      className="b--none bg-canvas ml2"
      aria-label="toggle menu"
      disabled={isDisabled}
      onClick={onClick}
    >
      <img src={iconHelp} width="14px" height="14px" alt="Open search help." />
    </button>
  );
};

export default ButtonHelp;
