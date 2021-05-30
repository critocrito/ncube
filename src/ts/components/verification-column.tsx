import c from "clsx";
import React from "react";

import ButtonDownload from "./button-download";

interface VerificationColumnProps {
  name: string;
  cntUnits: number;
  onDownload: () => Promise<void>;
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
      className={c(
        "w-80 border border-solitude flex flex-col",
        {
          "bg-white": !isRequiredColumn,
          "bg-canvas": isRequiredColumn,
          "opacity-40": !isDroppable,
          "shadow-md": isHighlighted && isDroppable,
        },
        className,
      )}
    >
      <div className="border-b border-solitude uppercase text-center w-full px-1.5 py-3 flex items-center">
        <h4 className="header4 font-bold">{name}</h4>
      </div>

      <div className="px-1.5 my-3 flex items-center justify-between">
        <div>ALL: {cntUnits}</div>
        <ButtonDownload onClick={onDownload} disabled={cntUnits === 0} />
      </div>

      <div className="p-1.5 h-full">{children}</div>
    </div>
  );
};

export default VerificationColumn;
