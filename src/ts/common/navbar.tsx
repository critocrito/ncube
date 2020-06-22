import React from "react";

const Navbar = () => {
  return (
    <div className="flex justify-between bg-white">
      <button
        className="link sapphire text-medium b--none pl2 pr2"
        onClick={() => window.history.back()}
      >
        &lt; Back
      </button>
      <div className="sapphire text-medium pl2 pr2 bg-fair-pink">
        Process Console
      </div>
    </div>
  );
};

export default Navbar;
