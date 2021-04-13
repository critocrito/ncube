import React, {useMemo} from "react";
import {Cell, Column} from "react-table";

import SourcesEmpty from "../../mdx/sources-empty.mdx";
import {voidFn} from "../lib/utils";
import {TableMachineInterpreter} from "../machines/table";
import {Source, SourceTag as Tag} from "../types";
import QueryTag from "./query-tag";
import SourceTag from "./source-tag";
import Table from "./table";

interface SourcesTableProps {
  sources: Source[];
  selected: Source[];
  total: number;
  table: TableMachineInterpreter;
  onClick: (source: Source) => void;
  onSelect: (sources: Source[]) => void;
  onDelete: (source: Source) => void;
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

const SourcesTable = ({
  sources,
  selected,
  total,
  table,
  onClick,
  onSelect = voidFn,
  onDelete = voidFn,
}: SourcesTableProps): JSX.Element => {
  const columns: Column<Source>[] = useMemo(
    () => [
      {
        Header: "Term",
        accessor: "term",
        Cell: ({value}: Cell) => decodeURI(String(value)),
      },

      {
        Header: "Type",
        accessor: "type",
        minWidth: 60,
        width: 60,
        maxWidth: 60,
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
        minWidth: 40,
        width: 40,
        maxWidth: 40,
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

  return sources.length === 0 ? (
    <>
      <SourcesEmpty />
    </>
  ) : (
    <>
      <Table<Source>
        name="sourcesTable"
        data={sources as Source[]}
        columns={columns}
        selected={selected as Source[]}
        total={total}
        table={table}
        onClick={onClick}
        onSelect={onSelect}
        onDelete={onDelete}
      />
    </>
  );
};

export default SourcesTable;
