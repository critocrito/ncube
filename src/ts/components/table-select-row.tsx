/* eslint react/jsx-props-no-spreading: off */
import React, {useEffect} from "react";
import {TableToggleAllRowsSelectedProps} from "react-table";

import {useCombinedRefs} from "../lib/hooks";

const TableSelectRow = React.forwardRef<
  HTMLInputElement,
  Partial<TableToggleAllRowsSelectedProps>
>(({indeterminate, ...rest}: Partial<TableToggleAllRowsSelectedProps>, ref) => {
  // eslint-disable-next-line unicorn/no-null
  const defaultRef = React.useRef<HTMLInputElement | null>(null);
  const combinedRef = useCombinedRefs<HTMLInputElement>(ref, defaultRef);

  useEffect(() => {
    if (combinedRef?.current) {
      combinedRef.current.indeterminate = indeterminate ?? false;
    }
  }, [combinedRef, indeterminate]);

  return (
    <>
      <input type="checkbox" ref={combinedRef} {...rest} />
    </>
  );
});

export default TableSelectRow;
