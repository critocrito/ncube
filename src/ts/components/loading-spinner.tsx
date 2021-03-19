import c from "classnames";
import React from "react";

interface LoadingSpinnerProps {
  className?: string;
}

const LoadingSpinner = ({className}: LoadingSpinnerProps) => {
  return (
    <div className={c("dib", className)}>
      <div className="loading loading-0" />
      <div className="loading loading-1" />
      <div className="loading loading-2" />
    </div>
  );
};

export default LoadingSpinner;
