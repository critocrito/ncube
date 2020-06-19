import React from "react";

interface OverlineProps {
  label: string;
}

const Overline = ({label}: OverlineProps) => {
  return <div className="b bb b--sapphire ttu sapphire pb">{`${label}:`}</div>;
};

export default Overline;
