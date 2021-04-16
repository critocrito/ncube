import React from "react";

import {voidFn} from "../lib/utils";
import {TableMachineInterpreter} from "../machines/table";
import {Segment, Unit} from "../types";
import ButtonHelp from "./button-help";
import DataActions from "./data-actions";
import DataSearchBar from "./data-search-bar";
import DataTable from "./data-table";

interface DataHomeProps {
  units: Unit[];
  query: string;
  total: number;
  table: TableMachineInterpreter;
  segment?: Segment;

  onCreateSegment?: () => void;
  onUpdateSegment?: () => void;
  onShow?: (unit: Unit) => void;
  onSearchQuery?: (query: string) => void;
  onSearch?: () => void;
  onHelp?: () => void;
}

const DataHome = ({
  units,
  query,
  total,
  table,
  segment,
  onCreateSegment = voidFn,
  onUpdateSegment = voidFn,
  onShow = voidFn,
  onSearchQuery = voidFn,
  onSearch = voidFn,
  onHelp = voidFn,
}: DataHomeProps) => {
  const isActionsDisabled = segment
    ? query === "" || query === segment.query || units.length === 0
    : query === "" || units.length === 0;

  return (
    <div className="flex flex-col">
      <div className="flex items-center w-1/2 my-3">
        <DataSearchBar
          query={query}
          onChange={onSearchQuery}
          onSearch={onSearch}
        />

        <ButtonHelp className="ml-2" onClick={onHelp} />
      </div>

      <DataActions
        hasSegment={segment !== undefined}
        isDisabled={isActionsDisabled}
        onCreateSegment={onCreateSegment}
        onUpdateSegment={onUpdateSegment}
      />

      <DataTable units={units} onClick={onShow} table={table} total={total} />
    </div>
  );
};

export default DataHome;
