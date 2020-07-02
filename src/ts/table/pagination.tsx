import c from "classnames";
import React, {PropsWithChildren, useCallback} from "react";

import {isNumber, paginate} from "../utils";

interface PaginationProps {
  pageIndex: number;
  pageSize: number;
  pageCount: number;
  total: number;
  loading: boolean;
  gotoPage: (page: number) => void;
}

const Pagination = ({
  pageIndex,
  pageSize,
  pageCount,
  total,
  gotoPage,
  loading,
}: PropsWithChildren<PaginationProps>) => {
  const start = pageIndex === 0 ? 1 : pageIndex * pageSize + 1;
  const end = start + pageSize - 1;
  const baseClass = "ml2 mr2 cursor";

  const counter =
    total > 0
      ? `Showing ${start}-${end} of ${total} items`
      : "Showing 0 of 0 items";

  const handleGotoPage = useCallback((page) => gotoPage(page), [gotoPage]);

  const pager = paginate(pageIndex, pageCount).map(
    (page: number | string, i: number) => {
      return isNumber(page) ? (
        <span
          key={page}
          onClick={() => handleGotoPage(page)}
          onKeyPress={() => handleGotoPage(page)}
          className={c(
            baseClass,
            "pointer",
            page === pageIndex ? "sapphire" : "gray-25",
          )}
          role="button"
          tabIndex={0}
        >
          {page + 1}
        </span>
      ) : (
        // eslint-disable-next-line react/no-array-index-key
        <span key={`${page}-${i}`} className={baseClass}>
          {page}
        </span>
      );
    },
  );

  return (
    <div className="mt3 text-medium gray-25">
      {pager} <span className="ml2 mr2" /> {loading ? "Loading" : counter}
    </div>
  );
};

export default Pagination;
