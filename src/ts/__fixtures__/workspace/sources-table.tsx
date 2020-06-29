import React, {useState} from "react";

import {Source} from "../../types";
import SourcesTable from "../../workspace/sources-table";
import data from "./sources.json";

const Wrapper = () => {
  const [deleted, setDeleted] = useState<string | undefined>(undefined);
  const [selected, setSelected] = useState<string[] | undefined>(undefined);

  return (
    <div>
      <SourcesTable
        sources={data}
        onCreate={() => console.log("CREATE")}
        onDelete={(source: Source) => setDeleted(source.term)}
        handleSelected={setSelected}
      />
      {deleted === undefined ? <div /> : <div>Deleted: {deleted}</div>}
      {selected === undefined ? (
        <div />
      ) : (
        <div>Selected: {selected.join(", ")}</div>
      )}
    </div>
  );
};

export default (
  <div className="noto lh-copy pa2 flex flex-column w-70">
    <div className="ml4">
      <Wrapper />
    </div>
  </div>
);
