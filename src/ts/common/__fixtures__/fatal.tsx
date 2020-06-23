import React from "react";

import Fatal from "../fatal";

const Wrapper = () => {
  return (
    <div>
      <Fatal msg="I'm a teapot." reset={() => {}} />
    </div>
  );
};

export default (
  <div className="noto lh-copy pa2 flex flex-column bg-canvas">
    <Wrapper />
  </div>
);
