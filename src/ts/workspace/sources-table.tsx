import React, {useCallback, useMemo, useState} from "react";
import {Cell, Column} from "react-table";

import QueryTag from "../common/query-tag";
import SourceTag from "../common/source-tag";
import {listSources} from "../http";
import Table from "../table";
import SelectColumnFilter from "../table/select-filter";
import {Source, SourceTag as Tag, Workspace} from "../types";

interface SourcesTableProps {
  workspace: Workspace;
  total: number;
  onCreate: () => void;
  handleSelected: (ids: string[]) => void;
  onDelete: (source: Source) => void;
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

const SourcesTable = ({
  workspace,
  total,
  onCreate,
  onDelete,
  handleSelected,
}: SourcesTableProps) => {
  const [data, setData] = useState<Source[]>([]);
  const [loading, setLoading] = useState(false);
  const [pageCount, setPageCount] = useState(0);

  const fetchData = useCallback(
    async (pageIndex: number, pageSize: number) => {
      setLoading(true);
      setData(await listSources(workspace.slug, pageIndex, pageSize));
      setPageCount(Math.ceil(total / pageSize));
      setLoading(false);
    },
    [total, workspace],
  );

  const columns: Column<Source>[] = useMemo(
    () => [
      {
        Header: "Term",
        accessor: "term",
        filter: "fuzzyText",
        Cell: ({value}: Cell) => decodeURI(String(value)),
      },

      {
        Header: "Type",
        accessor: "type",
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

      {
        Header: "Tags",
        accessor: "tags",
        // Filter: SelectColumnFilter,
        // filter: "includes",
        disableFilters: true,
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

  // FIXME: Do I need to wrap the handlers in useCallback?

  return (
    <Table<Source>
      name="sourcesTable"
      handleSelected={handleSelected}
      onCreate={onCreate}
      onDelete={onDelete}
      data={data}
      columns={columns}
      fetchData={fetchData}
      loading={loading}
      pageCount={pageCount}
      total={total}
    />
  );
};

export default SourcesTable;
