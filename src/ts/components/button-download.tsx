import c from "clsx";
import React from "react";

import iconDownload from "../../../resources/public/images/icon_download.svg";

interface ButtonDownloadProps {
  onClick: () => Promise<void>;
  disabled?: boolean;
  label?: string;
  className?: string;
}

const ButtonDownload = ({
  onClick,
  disabled = false,
  label,
  className,
}: ButtonDownloadProps) => {
  const classes = c(
    "flex items-center border-none bg-transparent",
    {
      "pointer-events-none opacity-50": disabled,
    },
    className,
  );

  return (
    <button className={classes} onClick={onClick}>
      {label && <span>{label}</span>}
      <img src={iconDownload} className="h-5 w-5" alt="download file" />
    </button>
  );
};

export default ButtonDownload;
