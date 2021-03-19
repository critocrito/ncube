import React from "react";

import Header from "./header";

interface LayoutProps {
  children: JSX.Element;
}

const Layout = ({children}: LayoutProps) => {
  return (
    <div className="pa4">
      <Header />

      <div>{children}</div>
    </div>
  );
};

export default Layout;
