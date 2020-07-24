import React from "react";

import LoadingSpinner from "../../common/loading-spinner";

const Wrapper = () => {
  return (
    <div>
      <LoadingSpinner />
    </div>
  );
};

export default (
  <div className="noto lh-copy pa2 flex flex-column">
    <Wrapper />
  </div>
);
