import c from "clsx";
import React from "react";

import rpc from "../lib/rpc";

interface ExternalLinkProps {
  href: string;
  children: JSX.Element | string;
  className?: string;
}

const ExternalLink = ({href, children, className}: ExternalLinkProps) => {
  return (
    <button
      className={c(
        "b--transparent bg-transparent pointer text-bittersweet underline pa0",
        className,
      )}
      onClick={() => rpc.openExternal(href)}
    >
      {children}
    </button>
  );
};

export default ExternalLink;
