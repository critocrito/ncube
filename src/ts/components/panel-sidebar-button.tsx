import React from "react";

import chevronLeft from "../svg/chevron_left.svg";
import chevronRight from "../svg/chevron_right.svg";

interface PanelSidebarButtonProps {
  isExpanded: boolean;
  onClick: () => void;
}

const PanelSidebarButton = ({isExpanded, onClick}: PanelSidebarButtonProps) => {
  return (
    <button
      className="text-sapphire font-bold h-8 w-8 flex justify-around items-center rounded-full shadow-md bg-canvas dim"
      onClick={onClick}
    >
      {isExpanded ? (
        <img src={chevronLeft} alt="close sidebar" className="h1 w1" />
      ) : (
        <img src={chevronRight} alt="open sidebar" className="h1 w1" />
      )}
    </button>
  );
};

export default PanelSidebarButton;
