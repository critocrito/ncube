import c from "classnames";
import React from "react";

interface OverlineProps {
  label: string;
  className?: string;
}

const Overline = ({label, className}: OverlineProps) => {
  return (
    <div
      className={c("b bb b--sapphire ttu sapphire pb", className)}
    >{`${label}:`}</div>
  );
};

export default Overline;
