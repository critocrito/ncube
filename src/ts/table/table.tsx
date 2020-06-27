/* eslint react/jsx-props-no-spreading: off */
import c from "classnames";
import React, {PropsWithChildren, useMemo} from "react";
import {
  CellProps,
  FilterProps,
  HeaderProps,
  Hooks,
  Row,
  TableOptions,
  useFilters,
  useFlexLayout,
  usePagination,
  useRowSelect,
  useTable,
} from "react-table";

import ActionBar from "./action-bar";
import DeleteRow from "./delete-row";
import {fuzzyTextFilter} from "./filters";
import Pagination from "./pagination";
import SelectRow from "./select-row";

// Define a default UI for filtering
const DefaultColumnFilter = <T extends Record<string, unknown>>({
  column: {filterValue, preFilteredRows, setFilter},
}: FilterProps<T>) => {
  const count = preFilteredRows.length;

  return (
    <input
      value={filterValue || ""}
      onChange={(e) => {
        setFilter(e.target.value || undefined); // Set undefined to remove the filter entirely
      }}
      placeholder={`Search ${count} records...`}
    />
  );
};

export interface TableProps<T extends {id: number}> extends TableOptions<T> {
  name: string;
  onCreate?: () => void;
  onDelete?: (item: T) => void;
  handleSelected: (ids: string[]) => void;
}

const selectHook = <T extends {id: number}>(hooks: Hooks<T>) => {
  hooks.visibleColumns.push((columns) => [
    {
      id: "selection",
      minWidth: 5,
      width: 5,
      maxWidth: 5,

      Header: ({getToggleAllRowsSelectedProps}: HeaderProps<T>) => (
        <div>
          <SelectRow {...getToggleAllRowsSelectedProps()} />
        </div>
      ),

      Cell: ({row}: CellProps<T>) => {
        return (
          <div className="flex items-center justify-around">
            <div>
              <SelectRow {...row.getToggleRowSelectedProps()} />
            </div>
          </div>
        );
      },
    },
    ...columns,
  ]);
};

const hooks = [
  useFilters,
  useFlexLayout,
  usePagination,
  useRowSelect,
  selectHook,
];

const Table = <T extends {id: number}>({
  data,
  columns,
  handleSelected,
  onCreate,
  onDelete,
}: PropsWithChildren<TableProps<T>>) => {
  const baseClass = "ba b--gray-25";
  const cellClass = c(baseClass, "text-medium");

  const filterTypes = useMemo(
    () => ({
      fuzzyText: fuzzyTextFilter,
    }),
    [],
  );

  const defaultColumn = useMemo(
    () => ({
      Filter: DefaultColumnFilter,
    }),
    [],
  );

  const instance = useTable<T>(
    {
      columns,
      data,
      defaultColumn,
      filterTypes,
      initialState: {
        pageSize: 20,
      },
      getRowId: (row: T, _relativeIndex: number, _parent?: Row<T>): string =>
        row.id.toString(),
    },
    ...hooks,
  );

  const {
    getTableProps,
    headerGroups,
    getTableBodyProps,
    page,
    prepareRow,
  } = instance;

  return (
    <div className="flex flex-column">
      <ActionBar
        instance={instance}
        className="mb3"
        onCreate={onCreate}
        handleSelected={handleSelected}
      />

      <table className="w-100 collapse ba b--sapphire" {...getTableProps()}>
        <thead>
          {headerGroups.map((headerGroup) => (
            <tr {...headerGroup.getHeaderGroupProps()} className="bg-canvas">
              {headerGroup.headers.map((column) => (
                <th
                  className={c(baseClass, "b tl")}
                  {...column.getHeaderProps()}
                >
                  {column.render("Header")}
                  {column.canFilter ? (
                    <span className="ml2">{column.render("Filter")}</span>
                  ) : (
                    ""
                  )}
                </th>
              ))}
            </tr>
          ))}
        </thead>

        <tbody {...getTableBodyProps()}>
          {page.map((row) => {
            prepareRow(row);

            if (row.cells.length === 0) return <tr />;

            const firstCell = row.cells[0];
            const restCells = row.cells.slice(1);

            return (
              <tr {...row.getRowProps()}>
                <td className={cellClass} {...firstCell.getCellProps()}>
                  <div className="flex items-center">
                    {onDelete && (
                      <DeleteRow onDelete={() => onDelete(row.original)} />
                    )}
                    {firstCell.render("Cell")}
                  </div>
                </td>

                {restCells.map((cell) => {
                  return (
                    <td className={cellClass} {...cell.getCellProps()}>
                      {cell.render("Cell")}
                    </td>
                  );
                })}
              </tr>
            );
          })}
        </tbody>
      </table>

      <Pagination instance={instance} />
    </div>
  );
};

export default Table;
