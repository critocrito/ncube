import React, {useMemo} from "react";
import {Cell, Column} from "react-table";

import {truncate, voidFn} from "../lib/utils";
import {TableMachineInterpreter} from "../machines/table";
import {SourceTag as Tag, Unit} from "../types";
import QueryTag from "./query-tag";
import SourceTag from "./source-tag";
import Table from "./table";

interface DataTableProps {
  units: Unit[];
  table: TableMachineInterpreter;
  total: number;
  onClick: (unit: Unit) => void;
}

const mapToKind = (type: string): "youtube" | "twitter" | "http" => {
  switch (true) {
    case type.startsWith("youtube"):
      return "youtube";

    case type.startsWith("twitter"):
      return "twitter";

    default:
      return "http";
  }
};

const DataTable = ({units, table, total, onClick = voidFn}: DataTableProps) => {
  const selected: Unit[] = [];

  const columns: Column<Unit>[] = useMemo(
    () => [
      {
        Header: "Title",
        accessor: "title",
        Cell: ({value}: Cell) => (value ? truncate(value, 60) : ""),
      },

      {
        Header: "Description",
        accessor: "description",
        Cell: ({value}: Cell) => (value ? truncate(value, 80) : ""),
      },

      {
        Header: "Url",
        accessor: "href",
        Cell: ({value}: Cell) =>
          value ? truncate(decodeURI(String(value)), 80) : "",
      },

      {
        Header: "Source",
        accessor: "source",
        minWidth: 50,
        width: 50,
        maxWidth: 50,
        Cell: ({value}: Cell) => {
          const kind = mapToKind(value);
          return (
            <div className="flex justify-around">
              <SourceTag kind={kind} />
            </div>
          );
        },
      },

      {
        Header: "Tags",
        accessor: "tags",
        minWidth: 110,
        width: 110,
        maxWidth: 110,
        Cell: ({value}: Cell) => {
          return (
            <div className="flex justify-around">
              {value.map((tag: Tag) => (
                <QueryTag
                  key={tag.label}
                  label={tag.label}
                  description={tag.description}
                />
              ))}
            </div>
          );
        },
      },
    ],
    [],
  );

  return (
    <Table<Unit>
      name="dataTable"
      data={units}
      selected={selected}
      columns={columns}
      total={total}
      table={table}
      onClick={onClick}
      onSelect={() => {}}
    />
  );
};

export default DataTable;
