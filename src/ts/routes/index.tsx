import {Router} from "@reach/router";
import React from "react";

import Home from "./home";
import Onboarding from "./onboarding";

const Routes = () => {
  return (
    <Router>
      <Home path="/" />
      <Onboarding path="onboarding" />
    </Router>
  );
};

export default Routes;
