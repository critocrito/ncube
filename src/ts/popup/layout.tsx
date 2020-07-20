import c from "classnames";
import React from "react";

import {SourceReq} from "../types";
import Header from "./header";

interface LayoutProps {
  sourceReq: SourceReq;
  children: JSX.Element;
}

const Layout = ({children, sourceReq}: LayoutProps) => {
  let platform: "youtube" | "twitter" | "http";

  switch (true) {
    case sourceReq.type.startsWith("youtube"): {
      platform = "youtube";
      break;
    }

    case sourceReq.type.startsWith("twitter"): {
      platform = "twitter";
      break;
    }

    default:
      platform = "http";
  }

  return (
    <div>
      <Header />

      <div className="flex flex-column w-100">
        <div data-testid="color-coding" className={c("h1", `bg-${platform}`)}>
          &nbsp;
        </div>
        <div className="pa2">{children}</div>
      </div>
    </div>
  );
};

export default Layout;
