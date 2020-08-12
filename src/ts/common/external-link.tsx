import c from "classnames";
import React from "react";

import rpc from "../rpc";

interface ExternalLinkProps {
  url: string;
  children: JSX.Element | string;
  className?: string;
}

const ExternalLink = ({url, children, className}: ExternalLinkProps) => {
  return (
    <button
      className={c("b--transparent bg-transparent pointer", className)}
      onClick={() => rpc.openExternal(url)}
    >
      {children}
    </button>
  );
};

export default ExternalLink;
