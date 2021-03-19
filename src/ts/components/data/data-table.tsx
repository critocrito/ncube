import {useMachine} from "@xstate/react";
import c from "classnames";
import React, {useCallback, useEffect, useMemo, useState} from "react";
import {Cell, Column} from "react-table";

import HelpText from "../../../mdx/search-help.mdx";
import CreateSegmentForm from "../../forms/create-segment";
import UpdateSegmentForm from "../../forms/update-segment";
import {
  createSegment,
  listUnits,
  searchUnits,
  updateSegment,
} from "../../lib/http";
import {truncate, useServiceLogger} from "../../lib/utils";
import machine, {TableEventSearch} from "../../machines/table";
import {Segment, SourceTag as Tag, Unit, Workspace} from "../../types";
import Button from "../button";
import Error from "../error";
import Fatal from "../fatal";
import FormHandler from "../form-handler";
import Modal from "../modal";
import QueryTag from "../query-tag";
import SourceTag from "../source-tag";
import Table from "../table";
import DataDetails from "./details";
import SearchBar from "./search-bar";

interface DataTableProps {
  workspace: Workspace;
  totalStat: number;
  segment?: Segment;
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

const DataTable = ({workspace, totalStat, segment}: DataTableProps) => {
  // We separate the query from the search query. When actually searching we use
  // the searchQuery. The query context field contains the current query.
  const [searchQuery, setSearchQuery] = useState(segment ? segment.query : "");
  const [state, send, service] = useMachine(machine, {
    services: {
      listItems: async ({query}, ev) => {
        const {pageIndex, pageSize} = ev as TableEventSearch;

        if (query === "") {
          const units = await listUnits(workspace.slug, pageIndex, pageSize);
          return {data: units, total: totalStat};
        }

        return searchUnits(workspace.slug, query, pageIndex, pageSize);
      },
    },

    context: {
      query: searchQuery,
      pageIndex: 0,
      pageSize: 20,
      results: [],
      selected: [],
      total: totalStat,
    },
  });

  useServiceLogger(service, machine.id);

  const {
    error,
    total,
    results,
    selected,
    query,
    pageIndex,
    pageSize,
  } = state.context;

  const fetchData = useCallback(
    async (index: number, size: number) => {
      if (searchQuery && searchQuery !== "")
        send("SEARCH", {query: searchQuery, pageIndex: index, pageSize: size});
    },
    [send, searchQuery],
  );

  // Force the initial fetch of data.
  useEffect(() => {
    send("SEARCH", {query: searchQuery, pageIndex, pageSize});
  }, [send, searchQuery, pageIndex, pageSize]);

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

  const handleDetails = useCallback(
    (unit: Unit) => send("SHOW_DETAILS", {item: unit}),
    [send],
  );

  const handleSelect = useCallback(
    (units: Unit[]) => send("SET_SELECTION", {selected: units}),
    [send],
  );

  const handleHelp = useCallback(() => send("HELP"), [send]);

  const searchBar = (
    <div className="w-50 mt2 mb2">
      <SearchBar
        query={query}
        onChange={(q) => send("SET_QUERY", {query: q})}
        onSearch={(q) => setSearchQuery(q)}
        onHelp={handleHelp}
      />
    </div>
  );

  const segmentButton = (
    <div className="flex items-center mb3 ml-auto">
      {segment ? (
        <Button
          onClick={() => send("CREATE")}
          size="large"
          disabled={query === "" || results.length === 0}
        >
          Update Segment
        </Button>
      ) : (
        <Button
          onClick={() => send("CREATE")}
          size="large"
          disabled={query === "" || results.length === 0}
        >
          Save Segment
        </Button>
      )}
    </div>
  );

  const table = (
    <Table<Unit>
      name="dataTable"
      data={results as Unit[]}
      selected={selected as Unit[]}
      columns={columns}
      fetchData={fetchData}
      total={total}
      controlledPageIndex={state.context.pageIndex}
      controlledPageSize={state.context.pageSize}
      onDetails={handleDetails}
      onSelect={handleSelect}
    />
  );

  switch (true) {
    // eslint-disable-next-line no-fallthrough
    case state.matches("fetching"):
    case state.matches("table"): {
      const loading = !!state.matches("fetching");

      return (
        <div
          className={c(
            "flex flex-column",
            loading ? "o-40 no-hover" : undefined,
          )}
        >
          {searchBar}

          {segmentButton}

          {table}
        </div>
      );
    }

    case state.matches("help"): {
      return (
        <div>
          <Modal
            onCancel={() => send("SHOW_TABLE")}
            title="Confirm"
            description="Please fill in any missing data."
          >
            <HelpText />
          </Modal>

          <div className="flex flex-column">
            {searchBar}

            {segmentButton}

            {table}
          </div>
        </div>
      );
    }

    case state.matches("details"):
      switch (state.event.type) {
        case "SHOW_DETAILS": {
          return (
            <div>
              <Modal
                onCancel={() => send("SHOW_TABLE")}
                title="Confirm"
                description="Describing this modal"
              >
                <DataDetails unit={state.event.item as Unit} />
              </Modal>
              <div className="flex flex-column">
                {searchBar}

                {segmentButton}

                {table}
              </div>
            </div>
          );
        }

        default:
          return (
            <Fatal
              msg={`Source route didn't match any valid state: ${state.value}`}
              reset={() => send("RETRY")}
            />
          );
      }

    case state.matches("create"):
      return (
        <div>
          <Modal
            onCancel={() => send("SHOW_TABLE")}
            title="Confirm"
            description="Please fill in any missing data."
          >
            <div className="flex flex-column">
              <p>
                {segment
                  ? "Modify this segment"
                  : "Add a new segment for your workspace."}
              </p>

              <FormHandler
                initialValues={
                  segment ? {query, title: segment.title} : {query}
                }
                onSave={(values) =>
                  segment
                    ? updateSegment(workspace.slug, segment.slug, values)
                    : createSegment(workspace.slug, values)
                }
                onDone={() => send("SHOW_TABLE")}
                Form={segment ? UpdateSegmentForm : CreateSegmentForm}
                workspace={workspace}
              />
            </div>
          </Modal>

          <div className="flex flex-column">
            {searchBar}

            {segmentButton}

            {table}
          </div>
        </div>
      );

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
          msg={`Source route didn't match any valid state: ${state.value}`}
          reset={() => send("RETRY")}
        />
      );
  }
};

export default DataTable;
