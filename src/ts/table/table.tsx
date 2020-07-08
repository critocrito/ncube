/* eslint react/jsx-props-no-spreading: off */
import c from "classnames";
import React, {PropsWithChildren, useEffect} from "react";
import {
  CellProps,
  HeaderProps,
  Hooks,
  IdType,
  Row,
  TableOptions,
  useFlexLayout,
  usePagination,
  useRowSelect,
  useTable,
} from "react-table";

import DeleteRow from "./delete-row";
import Pagination from "./pagination";
import SelectRow from "./select-row";

export interface TableProps<T extends {id: number}> extends TableOptions<T> {
  name: string;
  selected: T[];
  total: number;
  controlledPageIndex: number;
  controlledPageSize: number;
  fetchData: (page: number, size: number) => void;
  loading?: boolean;
  query?: string;
  onSelect: (items: T[]) => void;
  onDetails?: (item: T) => void;
  onDelete?: (item: T) => void;
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

const hooks = [useFlexLayout, usePagination, useRowSelect, selectHook];

const Table = <T extends {id: number}>({
  data,
  selected,
  columns,
  total,
  controlledPageIndex,
  controlledPageSize,
  fetchData,
  onSelect,
  onDetails,
  onDelete,
  loading = false,
}: PropsWithChildren<TableProps<T>>) => {
  const controlledPageCount = Math.ceil(total / controlledPageSize);

  const baseClass = "ba b--gray-25";
  const cellClass = c(baseClass, "text-medium");

  const {
    getTableProps,
    headerGroups,
    getTableBodyProps,
    page,
    prepareRow,
    gotoPage,
    selectedFlatRows,
    state: {pageIndex, pageSize},
  } = useTable<T>(
    {
      columns,
      data,
      initialState: {
        pageIndex: controlledPageIndex,
        pageSize: controlledPageSize,
        selectedRowIds: selected.reduce(
          (memo, {id}) => Object.assign(memo, {[id]: true}),
          {} as Record<IdType<T>, boolean>,
        ),
      },
      manualPagination: true,
      autoResetPage: false,
      autoResetSelectedRows: false,
      pageCount: controlledPageCount,
      getRowId: (row: T, _relativeIndex: number, _parent?: Row<T>): string =>
        row.id.toString(),
    },
    ...hooks,
  );

  useEffect(() => {
    // We avoid unnecessary requests and rerenders
    if (pageIndex !== controlledPageIndex || pageSize !== controlledPageSize)
      fetchData(pageIndex, pageSize);
  }, [fetchData, pageIndex, pageSize, controlledPageIndex, controlledPageSize]);

  useEffect(() => {
    if (selected.length !== selectedFlatRows.length) {
      onSelect(selectedFlatRows.map((row: Row<T>) => row.original));
    }
  }, [onSelect, selectedFlatRows, selected]);

  return (
    <div>
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
                      <div
                        role="button"
                        tabIndex={0}
                        onClick={() =>
                          onDetails ? onDetails(row.original) : {}
                        }
                        onKeyPress={() =>
                          onDetails ? onDetails(row.original) : {}
                        }
                      >
                        {cell.render("Cell")}
                      </div>
                    </td>
                  );
                })}
              </tr>
            );
          })}
        </tbody>
      </table>

      <Pagination
        pageIndex={controlledPageIndex}
        pageSize={controlledPageSize}
        pageCount={controlledPageCount}
        total={total}
        loading={loading}
        gotoPage={(p: number) => gotoPage(p)}
      />
    </div>
  );
};

export default Table;
