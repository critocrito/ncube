import React, {useCallback, useMemo, useState} from "react";
import {Cell, Column} from "react-table";

import SourceTag from "../common/source-tag";
import {listUnits} from "../http";
import Table from "../table";
import SelectColumnFilter from "../table/select-filter";
import {Unit, Workspace} from "../types";

interface DataTableProps {
  workspace: Workspace;
  total: number;
  handleSelected: (ids: string[]) => void;
}

const mapToKind = (type: string): "youtube" | "twitter" | "url" => {
  switch (true) {
    case type.startsWith("youtube"):
      return "youtube";

    case type.startsWith("twitter"):
      return "twitter";

    default:
      return "url";
  }
};

const DataTable = ({workspace, total, handleSelected}: DataTableProps) => {
  const [data, setData] = useState<Unit[]>([]);
  const [loading, setLoading] = useState(false);
  const [pageCount, setPageCount] = useState(0);

  const fetchData = useCallback(
    async (pageIndex: number, pageSize: number) => {
      setLoading(true);
      setData(await listUnits(workspace.slug, pageIndex, pageSize));
      setPageCount(Math.ceil(total / pageSize));
      setLoading(false);
    },
    [total, workspace],
  );

  const columns: Column<Unit>[] = useMemo(
    () => [
      {
        Header: "ID",
        accessor: "id",
      },

      {
        Header: "Url",
        accessor: "href",
        filter: "fuzzyText",
        Cell: ({value}: Cell) => (value ? decodeURI(String(value)) : ""),
      },

      {
        Header: "Source",
        accessor: "source",
        Filter: SelectColumnFilter,
        filter: "includes",
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
    ],
    [],
  );

  // FIXME: Do I need to wrap the handlers in useCallback?

  return (
    <Table<Unit>
      name="sourcesTable"
      handleSelected={handleSelected}
      data={data}
      columns={columns}
      fetchData={fetchData}
      loading={loading}
      pageCount={pageCount}
      total={total}
    />
  );
};

export default DataTable;
