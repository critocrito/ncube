import c from "classnames";
import React from "react";

import rpc from "../rpc";

interface ExternalLinkProps {
  href: string;
  children: JSX.Element | string;
  className?: string;
}

const ExternalLink = ({href, children, className}: ExternalLinkProps) => {
  return (
    <button
      className={c(
        "b--transparent bg-transparent pointer bittersweet underline pa0",
        className,
      )}
      onClick={() => rpc.openExternal(href)}
    >
      {children}
    </button>
  );
};

export default ExternalLink;
