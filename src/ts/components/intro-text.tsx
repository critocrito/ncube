import c from "clsx";
import React from "react";

interface IntroTextProps {
  children: JSX.Element;
  className?: string;
}

const IntroText = ({children, className}: IntroTextProps) => {
  return <div className={c("mb-4", className)}>{children}</div>;
};

export default IntroText;
