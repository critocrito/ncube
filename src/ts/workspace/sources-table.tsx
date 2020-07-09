import {useMachine} from "@xstate/react";
import c from "classnames";
import React, {useCallback, useEffect, useMemo} from "react";
import {Cell, Column} from "react-table";

import Button from "../common/button";
import Error from "../common/error";
import Fatal from "../common/fatal";
import FormHandler from "../common/form-handler";
import Modal from "../common/modal";
import QueryTag from "../common/query-tag";
import SourceTag from "../common/source-tag";
import {useAppCtx} from "../context";
import CreateSourceForm, {CreateSourceFormValues} from "../forms/create-source";
import {createSource, listSources, removeSource, searchSources} from "../http";
import machine from "../machines/table";
import Table from "../table";
import ActionBar from "../table/action-bar";
import {Source, SourceTag as Tag, Workspace} from "../types";
import {useServiceLogger} from "../utils";

interface SourcesTableProps {
  workspace: Workspace;
  totalStat: number;
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

const saveSource = (
  slug: string,
  values: CreateSourceFormValues,
): Promise<void> => {
  return createSource(slug, values);
};

const SourcesTable = ({workspace, totalStat}: SourcesTableProps) => {
  const [state, send, service] = useMachine(machine, {
    services: {
      listItems: async ({query, pageIndex, pageSize}, _ev) => {
        if (query === "") {
          const sources = await listSources(
            workspace.slug,
            pageIndex,
            pageSize,
          );
          return {data: sources, total: totalStat};
        }
        return searchSources(workspace.slug, query, pageIndex, pageSize);
      },

      deleteItem: (_ctx, {id}) => removeSource(workspace.slug, id),
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

  const [, appSend] = useAppCtx();

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
    (index: number, size: number) => {
      send("SEARCH", {query, pageIndex: index, pageSize: size});
    },
    [send, query],
  );

  // Force the initial fetch of data.
  useEffect(() => {
    fetchData(pageIndex, pageSize);
  }, [fetchData, pageIndex, pageSize]);

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

  const handleCreate = useCallback(() => send("CREATE"), [send]);

  const handleDelete = useCallback((item: Source) => send("DELETE", {item}), [
    send,
  ]);

  const actionBar = (
    <ActionBar
      selected={selected}
      onProcessSelected={() => console.log(selected)}
      onCreate={handleCreate}
    />
  );

  const table = (
    <Table<Source>
      name="sourcesTable"
      data={results as Source[]}
      columns={columns}
      selected={selected as Source[]}
      total={total}
      controlledPageIndex={state.context.pageIndex}
      controlledPageSize={state.context.pageSize}
      fetchData={fetchData}
      onDetails={handleDetails}
      onSelect={handleSelect}
      onDelete={handleDelete}
    />
  );

  const loading = !!state.matches("fetching") || !!state.matches("deleting");

  switch (true) {
    case state.matches("fetching"):
    case state.matches("deleting"):
    case state.matches("table"): {
      return (
        <div
          className={c(
            "flex flex-column",
            loading ? "o-40 no-hover" : undefined,
          )}
        >
          {actionBar}

          {table}
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
                {actionBar}

                {table}
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

    case state.matches("create"):
      switch (state.event.type) {
        case "CREATE": {
          return (
            <div>
              <Modal
                onCancel={() => send("SHOW_TABLE")}
                title="Confirm"
                description="Describing this modal"
              >
                <div className="flex flex-column">
                  <p>Add a new data source for your workspace.</p>

                  <FormHandler
                    onSave={(values) => saveSource(workspace.slug, values)}
                    onDone={() => send("SHOW_TABLE")}
                    Form={CreateSourceForm}
                    workspace={workspace}
                  />
                </div>
              </Modal>

              <div className="flex flex-column">
                {actionBar}

                {table}
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

    case state.matches("delete"):
      switch (state.event.type) {
        case "DELETE": {
          const {type, term, id} = state.event.item as Source;

          return (
            <div>
              <Modal
                onCancel={() => send("CANCEL")}
                title="Confirm delete."
                description={`Confirm you want to delete ${term}.`}
              >
                <div className="flex flex-column">
                  <p>Are you sure you want to delete the following source?</p>

                  <dl className="pa4 mt0 sapphire">
                    <dt className="f6 b">Type</dt>
                    <dd className="ml0">{type}</dd>
                    <dt className="f6 b mt2">Term</dt>
                    <dd className="ml0">{term}</dd>
                  </dl>

                  <div className="flex justify-between mt3 ml-auto">
                    <Button
                      className="mr3"
                      type="reset"
                      size="large"
                      kind="secondary"
                      onClick={() => send("CANCEL")}
                    >
                      Cancel
                    </Button>

                    <Button
                      className="mr3 fr"
                      type="submit"
                      size="large"
                      onClick={() =>
                        send("CONFIRM_DELETE", {
                          id,
                        })
                      }
                    >
                      Delete Source
                    </Button>
                  </div>
                </div>
              </Modal>

              <div className="flex flex-column">
                {actionBar}

                {table}
              </div>
            </div>
          );
        }

        default:
          return (
            <Fatal
              msg={`An unknown event is triggering this state: ${state.event.type}`}
              reset={() => appSend("RESTART_APP")}
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
