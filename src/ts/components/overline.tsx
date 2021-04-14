import c from "clsx";
import React from "react";

interface OverlineProps {
  label: string;
  className?: string;
}

const Overline = ({label, className}: OverlineProps) => {
  return (
    <div
      className={c(
        "font-bold border-b border-sapphire uppercase text-sapphire",
        className,
      )}
    >{`${label}:`}</div>
  );
};

export default Overline;
