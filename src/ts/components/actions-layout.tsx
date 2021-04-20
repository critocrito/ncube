import c from "clsx";
import React from "react";

interface ActionsLayoutProps {
  align?: "left" | "right";
  children: React.ReactNode;
}

const ActionsLayout = ({align = "left", children}: ActionsLayoutProps) => {
  const className = c("mt-3 flex space-x-4", {
    "justify-end": align === "right",
  });

  return <div className={className}>{children}</div>;
};

export default ActionsLayout;
