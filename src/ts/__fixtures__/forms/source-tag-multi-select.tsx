import React from "react";

import SourceTagMultiSelect from "../../forms/source-tag-multi-select";
import data from "./source-tags.json";

const Wrapper = () => {
  return (
    <div className="w-30">
      <SourceTagMultiSelect
        data={data}
        onRemove={console.log}
        onAdd={console.log}
      />
    </div>
  );
};

export default (
  <div className="noto lh-copy pa2 flex flex-column bg-canvas">
    <Wrapper />
  </div>
);
