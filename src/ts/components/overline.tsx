import c from "clsx";
import React from "react";

interface OverlineProps {
  label: string;
  className?: string;
}

const Overline = ({label, className}: OverlineProps) => {
  return (
    <div
      className={c("b bb b--sapphire ttu text-sapphire pb", className)}
    >{`${label}:`}</div>
  );
};

export default Overline;
