import c from "clsx";
import React from "react";

import iconHelp from "../../../resources/public/images/icon_help.svg";

interface ButtonHelpProps {
  onClick: () => void;
  isDisabled?: boolean;
  className?: string;
}

const ButtonHelp = ({
  onClick,
  isDisabled = false,
  className,
}: ButtonHelpProps) => {
  return (
    <button
      className={c(className)}
      aria-label="toggle menu"
      disabled={isDisabled}
      onClick={onClick}
    >
      <img className="w-5 h-5" src={iconHelp} alt="Open search help." />
    </button>
  );
};

export default ButtonHelp;
