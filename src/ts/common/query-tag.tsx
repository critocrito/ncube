import c from "classnames";
import React from "react";

interface QueryTagProps {
  label: string;
  description?: string;
  className?: string;
}

const QueryTag = ({label, className}: QueryTagProps) => {
  return (
    <div
      className={c(
        "tag mw5 flex justify-between items-center bg-solitude nowrap",
        className,
      )}
    >
      <div className="w-90 tc sapphire">
        <span>{label}</span>
      </div>
    </div>
  );
};

export default QueryTag;
