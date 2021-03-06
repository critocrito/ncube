import React from "react";

import logo from "../../../resources/public/images/logo_horizontal.svg";
import WorkspacesIntroduction from "../../mdx/workspaces-intro.mdx";
import ExternalLink from "./external-link";
import IntroText from "./intro-text";

const DashboardHeader = () => {
  return (
    <>
      <header className="mb-5 mt-4">
        <ExternalLink href="https://sugarcubetools.net">
          <img src={logo} alt="Ncube Logo" />
        </ExternalLink>
      </header>

      <IntroText>
        <WorkspacesIntroduction />
      </IntroText>
    </>
  );
};

export default DashboardHeader;
