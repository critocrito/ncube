import c from "classnames";
import React from "react";

interface IntroTextProps {
  children: JSX.Element;
  className?: string;
}

const IntroText = ({children, className}: IntroTextProps) => {
  return <div className={c("mb4", className)}>{children}</div>;
};

export default IntroText;
