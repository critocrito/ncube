import React from "react";

interface CounterProps {
  cnt: number;
}

const Counter = ({cnt}: CounterProps) => {
  return <p>{cnt}</p>;
};

export default Counter;
