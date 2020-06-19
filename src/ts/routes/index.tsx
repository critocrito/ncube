import {Redirect, Router} from "@reach/router";
import React from "react";

import Home from "./home";
import Onboarding from "./onboarding";
import Workspace from "./workspace";

const Routes = () => {
  return (
    <Router>
      <Redirect from="/" to="onboarding" />
      <Onboarding path="onboarding" />
      <Home path="w" />
      <Workspace path="w/:slug" />
    </Router>
  );
};

export default Routes;
