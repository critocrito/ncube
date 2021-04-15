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
  const classes = c(
    "w-20 text-sm text-center bg-solitude px-3 py-0.5 rounded-full truncate",
    className,
  );
  return <span className={classes}>{label}</span>;
};

export default QueryTag;
