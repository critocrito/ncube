import React from "react";

import {isString} from "../utils";

interface BasicPanelProps {
  header?: string;
  description?: string;
  children: JSX.Element;
}

const BasicPanel = ({children, header, description}: BasicPanelProps) => {
  const head = isString(header) ? <h1 className="header1">{header}</h1> : "";
  const desc = isString(description) ? <p>{description}</p> : "";
  return (
    <div className="mw8 center">
      <div className="cf w-100 pa2">
        {head}
        {desc}
        {children}
      </div>
    </div>
  );
};

export default BasicPanel;
