import React from "react";

import {isString} from "../lib/utils";

interface BasicPanelProps {
  header?: string;
  description?: string;
  children: JSX.Element;
}

const BasicPanel = ({children, header, description}: BasicPanelProps) => {
  const head = isString(header) ? <h1 className="header1">{header}</h1> : "";
  const desc = isString(description) ? <p>{description}</p> : "";
  return (
    <div className="container mx-auto max-w-6xl p-3">
      {head}
      {desc}
      {children}
    </div>
  );
};

export default BasicPanel;
