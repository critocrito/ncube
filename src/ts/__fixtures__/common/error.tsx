import React from "react";

import Error from "../../common/error";

const Wrapper = () => {
  return (
    <div>
      <Error msg="I'm a teapot." recover={() => {}} />
    </div>
  );
};

export default (
  <div className="noto lh-copy pa2 flex flex-column bg-canvas">
    <Wrapper />
  </div>
);
