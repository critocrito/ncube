import React from "react";

import logo from "../../../resources/public/images/icon_logo_discovery.svg";

const Header = () => {
  return (
    <header className="flex items-center w-100 pa2">
      <img alt="Ncube Discovery Logo" src={logo} />
      <div className="flex flex-column">
        <h1 className="ml3 mb0 pt2 ttu f3 lh-title">Ncube Discovery</h1>
      </div>
    </header>
  );
};

export default Header;
