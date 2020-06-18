import {Redirect, Router} from "@reach/router";
import React from "react";

import Home from "./home";
import Onboarding from "./onboarding";

const Routes = () => {
  return (
    <Router>
      <Redirect from="/" to="onboarding" />
      <Onboarding path="onboarding" />
      <Home path="w" />
    </Router>
  );
};

export default Routes;
