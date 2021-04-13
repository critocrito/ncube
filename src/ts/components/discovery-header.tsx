import React from "react";

import logo from "../../../resources/public/images/logo_discovery_text.svg";

const Header = () => {
  return (
    <header className="flex items-center w-100">
      <img alt="Ncube Discovery Logo" src={logo} />
    </header>
  );
};

export default Header;
