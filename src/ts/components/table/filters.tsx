import {matchSorter} from "match-sorter";
import {FilterValue, IdType, Row} from "react-table";

export const fuzzyTextFilter = <T extends Record<string, unknown>>(
  rows: Row<T>[],
  id: IdType<T>,
  filterValue: FilterValue,
) => {
  return matchSorter(rows, filterValue, {
    keys: [(row: Row<T>) => row.values[id]],
  });
};

// Let the table remove the filter if the string is empty
fuzzyTextFilter.autoRemove = (val: unknown) => !val;
