import c from "clsx";
import React from "react";

interface QueryTagProps {
  label: string;
  // FIXME: Either remove description prop or use it
  // eslint-disable-next-line react/no-unused-prop-types
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
      <div className="w-90 tc text-sapphire">
        <span>{label}</span>
      </div>
    </div>
  );
};

export default QueryTag;
