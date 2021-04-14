import React from "react";

import rpc from "../lib/rpc";

interface ExternalLinkProps {
  href: string;
  children: JSX.Element | string;
}

const ExternalLink = ({href, children}: ExternalLinkProps) => {
  return (
    <button
      className="text-bittersweet underline p-0"
      onClick={() => rpc.openExternal(href)}
    >
      {children}
    </button>
  );
};

export default ExternalLink;
