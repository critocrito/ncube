import c from "classnames";
import React, {useCallback, useMemo, useState} from "react";
import {Cell, Column} from "react-table";

import SourceTag from "../common/source-tag";
import {listUnits, searchUnits} from "../http";
import Table from "../table";
import SelectColumnFilter from "../table/select-filter";
import {Unit, Workspace} from "../types";

interface DataTableProps {
  workspace: Workspace;
  totalStat: number;
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

const DataTable = ({workspace, totalStat, handleSelected}: DataTableProps) => {
  const [data, setData] = useState<Unit[]>([]);
  const [loading, setLoading] = useState(false);
  const [total, setTotal] = useState(totalStat);

  const fetchData = useCallback(
    async (query: string, pageIndex: number, pageSize: number) => {
      setLoading(true);

      if (query === "") {
        setData(await listUnits(workspace.slug, pageIndex, pageSize));
        setTotal(totalStat);
      } else {
        const {data: units, total: newTotal} = await searchUnits(
          workspace.slug,
          query,
          pageIndex,
          pageSize,
        );
        setData(units);
        setTotal(newTotal);
      }

      setLoading(false);
    },
    [totalStat, workspace],
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
    <div
      className={c("flex flex-column", loading ? "o-40 no-hover" : undefined)}
    >
      <Table<Unit>
        name="sourcesTable"
        handleSelected={handleSelected}
        data={data}
        columns={columns}
        fetchData={fetchData}
        loading={loading}
        total={total}
        hasSearch
      />
    </div>
  );
};

export default DataTable;
