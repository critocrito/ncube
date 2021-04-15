import c from "clsx";
import React from "react";

import iconDownload from "../../../resources/public/images/icon_download.svg";

interface VerificationColumnProps {
  name: string;
  cntUnits: number;
  onDownload?: () => Promise<void>;
  children?: JSX.Element;
  isHighlighted?: boolean;
  isDroppable?: boolean;
  className?: string;
}

const VerificationColumn = ({
  name,
  cntUnits,
  children,
  onDownload,
  className,
  isHighlighted = false,
  isDroppable = true,
}: VerificationColumnProps) => {
  const isRequiredColumn = [
    "incoming_data",
    "discarded_data",
    "verified_data",
  ].includes(name);

  return (
    <div
      style={{width: "20rem"}}
      className={c(
        "ba b--solitude flex flex-column",
        className,
        isRequiredColumn ? "bg-canvas" : "bg-white",
        isDroppable ? undefined : "o-40",
        isHighlighted && isDroppable ? "shadow-4" : undefined,
      )}
    >
      <div className="bb b--solitude h3 ttu center w-100 pa2 mv2 flex items-center">
        <span className="text-md b text-sapphire">{name}</span>
      </div>

      <div className="pa2 mv2 flex items-center justify-between">
        <div>ALL: {cntUnits}</div>
        <button
          className={c(
            "pointer b--none bg-transparent",
            cntUnits === 0 ? "pointer-events-none o-50" : undefined,
          )}
          onClick={onDownload}
        >
          <img src={iconDownload} className="h2 w2" alt="download" />
        </button>
      </div>

      <div className="pa2 h-100">{children}</div>
    </div>
  );
};

export default VerificationColumn;
