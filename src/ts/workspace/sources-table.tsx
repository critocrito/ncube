import {useMachine} from "@xstate/react";
import c from "classnames";
import React, {useCallback, useEffect, useMemo} from "react";
import {Cell, Column} from "react-table";

import Error from "../common/error";
import Fatal from "../common/fatal";
import Modal from "../common/modal";
import QueryTag from "../common/query-tag";
import SourceTag from "../common/source-tag";
import {listSources, searchSources} from "../http";
import machine from "../machines/table";
import Table from "../table";
import ActionBar from "../table/action-bar";
import {Source, SourceTag as Tag, Workspace} from "../types";
import {useServiceLogger} from "../utils";

interface SourcesTableProps {
  workspace: Workspace;
  totalStat: number;
  onCreate: () => void;
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
  totalStat,
  onCreate,
  onDelete,
}: SourcesTableProps) => {
  const [state, send, service] = useMachine(machine, {
    services: {
      fetchData: async (_ctx, {query, pageIndex, pageSize}) => {
        if (query === "") {
          const units = await listSources(workspace.slug, pageIndex, pageSize);
          return {data: units, total: totalStat};
        }
        return searchSources(workspace.slug, query, pageIndex, pageSize);
      },
    },

    context: {
      query: "",
      pageIndex: 0,
      pageSize: 20,
      results: [],
      selected: [],
      total: totalStat,
    },
  });

  useServiceLogger(service, machine.id);

  const {error, total, results, selected, query} = state.context;

  const fetchData = useCallback(
    async (pageIndex: number, pageSize: number) => {
      send("SEARCH", {query, pageIndex, pageSize});
    },
    [send, query],
  );

  // Force the initial fetch of data.
  useEffect(() => {
    send("SEARCH", {
      query: state.context.query,
      pageIndex: state.context.pageIndex,
      pageSize: state.context.pageSize,
    });
  }, [send, state]);

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

  const handleDetails = useCallback(
    (source: Source) => send("SHOW_DETAILS", {item: source}),
    [send],
  );

  const handleSelect = useCallback(
    (sources: Source[]) => send("SET_SELECTION", {selected: sources}),
    [send],
  );

  switch (true) {
    case state.matches("fetch"):
    case state.matches("table"): {
      const loading = !!state.matches("fetch");

      return (
        <div
          className={c(
            "flex flex-column",
            loading ? "o-40 no-hover" : undefined,
          )}
        >
          <ActionBar
            selected={selected}
            onProcessSelected={() => console.log(selected)}
            onCreate={onCreate}
          />

          <Table<Source>
            name="sourcesTable"
            data={results as Source[]}
            selected={selected as Source[]}
            controlledPageIndex={state.context.pageIndex}
            controlledPageSize={state.context.pageSize}
            onDetails={handleDetails}
            onSelect={handleSelect}
            onDelete={onDelete}
            loading={loading}
            columns={columns}
            fetchData={fetchData}
            total={total}
          />
        </div>
      );
    }

    case state.matches("details"):
      switch (state.event.type) {
        case "SHOW_DETAILS": {
          const {id} = state.event.item;

          return (
            <div>
              <Modal
                onCancel={() => send("SHOW_TABLE")}
                title="Confirm"
                description="Describing this modal"
              >
                <div className="flex flex-column">{id}</div>
              </Modal>
              <div className="flex flex-column">
                <ActionBar
                  selected={selected}
                  onProcessSelected={() => console.log(selected)}
                  onCreate={onCreate}
                />

                <Table<Source>
                  name="sourcesTable"
                  data={results as Source[]}
                  selected={selected as Source[]}
                  controlledPageIndex={state.context.pageIndex}
                  controlledPageSize={state.context.pageSize}
                  onDetails={handleDetails}
                  onSelect={handleSelect}
                  columns={columns}
                  fetchData={fetchData}
                  total={total}
                />
              </div>
            </div>
          );
        }

        default:
          return (
            <Fatal
              msg={`Sources table didn't match any valid state: ${state.value}`}
              reset={() => send("RETRY")}
            />
          );
      }

    case state.matches("error"):
      return (
        <Error
          msg={error || "Failed to fetch sources."}
          recover={() => send("RETRY")}
        />
      );

    default:
      return (
        <Fatal
          msg={`Sources table didn't match any valid state: ${state.value}`}
          reset={() => send("RETRY")}
        />
      );
  }
};

export default SourcesTable;
