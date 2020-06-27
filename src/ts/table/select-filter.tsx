import React, {useMemo} from "react";
import {FilterProps} from "react-table";

// This is a custom filter UI for selecting
// a unique option from a list
const SelectColumnFilter = <T extends {id: number}>({
  column: {filterValue, setFilter, preFilteredRows, id},
}: FilterProps<T>) => {
  const filterOptions = useMemo(() => {
    const options: Set<string> = new Set();
    preFilteredRows.forEach((row) => {
      options.add(row.values[id]);
    });
    return [...options.values()];
  }, [id, preFilteredRows]);

  return (
    <select
      value={filterValue}
      onChange={(e) => {
        setFilter(e.target.value || undefined);
      }}
    >
      <option value="">All</option>
      {filterOptions.map((option) => (
        <option key={option} value={option}>
          {option}
        </option>
      ))}
    </select>
  );
};

export default SelectColumnFilter;
