import c from "clsx";
import React from "react";

const LoadingSpinner = () => {
  const className = "w-2 h-2 rounded bg-sapphire fl mr-1 animate-spinner";

  return (
    <div className="inline-block">
      <div className={c(className, "animate-delay-100")} />
      <div className={c(className, "animate-delay-300")} />
      <div className={c(className, "animate-delay-500")} />
    </div>
  );
};

export default LoadingSpinner;
