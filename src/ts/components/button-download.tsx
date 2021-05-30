import c from "clsx";
import React from "react";

import iconDownload from "../../../resources/public/images/icon_download.svg";

interface ButtonDownloadProps {
  onClick: () => Promise<void>;
  disabled: boolean;
  label?: string;
}

const ButtonDownload = ({onClick, disabled, label}: ButtonDownloadProps) => {
  const className = c("border-none bg-transparent", {
    "pointer-events-none opacity-50": disabled,
  });

  return (
    <button className={className} onClick={onClick}>
      {label && <span>label</span>}
      <img src={iconDownload} className="h-5 w-5" alt="download file" />
    </button>
  );
};

export default ButtonDownload;
