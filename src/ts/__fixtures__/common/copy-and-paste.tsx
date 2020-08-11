import React from "react";

import CopyAndPaste from "../../common/copy-and-paste";

const Wrapper = () => {
  return (
    <div className="flex flex-column">
      <div className="flex">
        <span className="mr3">Copy &apos;23&apos; into the clipboard</span>
        <CopyAndPaste value="23" />
      </div>
    </div>
  );
};

export default (
  <div className="noto lh-copy pa2 flex flex-column bg-canvas">
    <Wrapper />
  </div>
);
