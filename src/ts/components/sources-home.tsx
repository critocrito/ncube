import React from "react";

import {voidFn} from "../lib/utils";
import {TableMachineInterpreter} from "../machines/table";
import {Source} from "../types";
import SourcesActions from "./sources-actions";
import SourcesTable from "./sources-table";

interface SourcesHomeProps {
  sources: Source[];
  total: number;
  table: TableMachineInterpreter;
  onCreate?: () => void;
  onShow?: (source: Source) => void;
  onDelete?: (source: Source) => void;
}

const SourcesHome = ({
  sources,
  total,
  table,
  onCreate = voidFn,
  onShow = voidFn,
  onDelete = voidFn,
}: SourcesHomeProps) => {
  return (
    <>
      <SourcesActions onCreate={onCreate} />

      <SourcesTable
        sources={sources}
        selected={[]}
        total={total}
        table={table}
        onClick={onShow}
        onDelete={onDelete}
        onSelect={voidFn}
      />
    </>
  );
};

export default SourcesHome;
